
use std::net::SocketAddr;
use tokio_tcp::TcpStream;
use tokio_io::codec::Framed;
use tokio_io::AsyncRead;
use futures::prelude::*;

use *;
use protobuf_api::*;
use super::codec::{NnQ, NnR, NnCodec, RpcReq, RpcRsp, HandshakeContext, OpContext};


fn get_client_id() -> Vec<u8> {
    /*org.apache.hadoop.ipc.ClientId
      //The byte array of a UUID should be 16
      public static final int BYTE_LENGTH = 16;

      public static byte[] getClientId() {
        UUID uuid = UUID.randomUUID();
        ByteBuffer buf = ByteBuffer.wrap(new byte[BYTE_LENGTH]);
        buf.putLong(uuid.getMostSignificantBits());
        buf.putLong(uuid.getLeastSignificantBits());
        return buf.array();
      }
    */

    use uuid::Uuid;
    let uuid = Uuid::new_v4();
    Vec::from(&(uuid.as_bytes()[..]))
}

#[inline]
fn get_method_name(q: &NnQ) -> String {
    super::codec::get_method_name(q)
}

fn validate_response(response_header: RpcResponseHeaderProto,
                     client_id: &Vec<u8>, call_id: i32)
                     -> Result<()> {
    let (
        r_call_id, r_status, r_client_id, error_detail, error_msg, exception_class_name
    ) = pb_decons!(RpcResponseHeaderProto, response_header,
        call_id, status, client_id, error_detail, error_msg, exception_class_name
    );

    let _ = match r_status {
        RpcResponseHeaderProto_RpcStatusProto::SUCCESS =>
            Ok(()),
        st @ RpcResponseHeaderProto_RpcStatusProto::ERROR |
        st @ RpcResponseHeaderProto_RpcStatusProto::FATAL
        => Err(Error::RPC {
            protocol: "namenode".to_owned(),
            status: st,
            error_detail,
            error_msg: error_msg.to_owned(),
            exception_class_name: exception_class_name.to_owned()
        })
    }?;

    if r_call_id as i32 != call_id {
        Err(app_error!(nn "Call id mismatch, found {}, expected {}", r_call_id, call_id))
    } else if r_client_id != &client_id as &[u8] {
        Err(app_error!(nn "Client id mismatch, found {:?}, expected {:?}", r_client_id, client_id))
    } else{
        Ok(())
    }
}

#[derive(Debug)]
pub struct Connection {
    io: Framed<TcpStream, NnCodec>,
    client_id: Vec<u8>,
    call_id: i32
}

impl Connection {
    pub fn connect_det(addr: &SocketAddr, client_id: Vec<u8>, eff_user: String) -> BFI<Connection> {
        trace!("Trying to connect to {}, client_id={:?}, eff_user={}", addr, client_id, eff_user);
        let rv =
            TcpStream::connect(addr)
                .map(|c| Connection { io: c.framed(NnCodec::new()), client_id, call_id: 0 })
                .and_then(|c| c.send_handshake(eff_user));
        Box::new(rv)
    }

    pub fn connect(addr: &SocketAddr, eff_user: String) -> BFI<Connection> {
        Connection::connect_det(addr, get_client_id(), eff_user)
    }

    fn send_handshake(self, eff_user: String) -> BFI<Connection> {
        let Connection { io, client_id, call_id } = self;
        Box::new(
            io.send(RpcReq::Handshake(HandshakeContext::new(client_id.clone(), eff_user)))
                .map(move |c| Connection { io: c, client_id, call_id }))
    }
}

impl Stream for Connection {
    type Item = NnR;
    type Error = Error;
    fn poll(&mut self) -> Result<Async<Option<NnR>>> {
        match self.io.poll() {
            Ok(Async::NotReady) =>
                Ok(Async::NotReady),
            Ok(Async::Ready(Some(RpcRsp { header, payload }))) =>
                match (validate_response(header, &self.client_id, self.call_id), payload) {
                    (Ok(()), Some(p)) => {
                        self.call_id += 1;
                        Ok(Async::Ready(Some(p)))
                    }
                    (Ok(()), None) => Err(app_error!(other "Payload required but is empty").into()),
                    (Err(e), _) => Err(e.into())
                }
            Ok(Async::Ready(None)) =>
                Ok(Async::Ready(None)),
            Err(e) =>
                Err(e.into())
        }
    }
}

impl Sink for Connection {
    type SinkItem = NnQ;
    type SinkError = Error;
    fn start_send(&mut self, req: NnQ) -> Result<AsyncSink<NnQ>> {
        let method_name = get_method_name(&req);
        match self.io.start_send(RpcReq::Operation(
            OpContext {
                client_id: self.client_id.clone(),
                call_id: self.call_id,
                method_name
            },
            req
        )) {
            Ok(AsyncSink::Ready) => Ok(AsyncSink::Ready),
            Ok(AsyncSink::NotReady(RpcReq::Operation(_, t))) => Ok(AsyncSink::NotReady(t)),
            Err(e) => Err(e.into()),
            _ => Err(app_error!(other "Invalid rejected operation"))
        }
    }

    fn poll_complete(&mut self) -> Result<Async<()>> {
        self.io.poll_complete().map_err(|e| e.into())
    }

    fn close(&mut self) -> Result<Async<()>> {
        self.io.close().map_err(|e| e.into())
    }
}



#[test]
fn test_read_listing() {
    use util::test::ptk::*;
    let host_port = "127.0.0.1:58020";
    let t = spawn_test_server(host_port, test_script! {

    expect "68:72:70:63:09:00:00:00:00:00:1e:10:08:02:10:00:18:05:22:08:01:02:03:04:04:03:02:01:\
    0c:12:0a:0a:08:63:6c:6f:75:64:65:72:61:00:00:00:58:10:08:02:10:00:18:00:22:08:01:02:03:04:04:\
    03:02:01:3e:0a:0a:67:65:74:4c:69:73:74:69:6e:67:12:2e:6f:72:67:2e:61:70:61:63:68:65:2e:68:61:\
    64:6f:6f:70:2e:68:64:66:73:2e:70:72:6f:74:6f:63:6f:6c:2e:43:6c:69:65:6e:74:50:72:6f:74:6f:63:\
    6f:6c:18:01:07:0a:01:2f:12:00:18:00",

    send "00:00:01:70:12:08:00:10:00:18:09:3a:08:01:02:03:04:04:03:02:01:40:01:db:02:0a:d8:02:0a:\
    3d:08:01:12:0a:62:65:6e:63:68:6d:61:72:6b:73:18:00:22:03:08:ff:03:2a:04:68:64:66:73:32:0a:73:\
    75:70:65:72:67:72:6f:75:70:38:e1:d7:df:d6:d5:2b:40:00:50:00:58:00:68:8e:80:01:70:00:80:01:00:\
    0a:39:08:01:12:05:68:62:61:73:65:18:00:22:03:08:ed:03:2a:05:68:62:61:73:65:32:0a:73:75:70:65:\
    72:67:72:6f:75:70:38:b4:be:e4:95:f7:2b:40:00:50:00:58:00:68:8d:80:01:70:09:80:01:00:0a:31:08:\
    01:12:04:73:6f:6c:72:18:00:22:03:08:ed:03:2a:04:73:6f:6c:72:32:04:73:6f:6c:72:38:e1:91:e8:d6:\
    d5:2b:40:00:50:00:58:00:68:f9:81:01:70:00:80:01:00:0a:36:08:01:12:03:74:6d:70:18:00:22:03:08:\
    ff:07:2a:04:68:64:66:73:32:0a:73:75:70:65:72:67:72:6f:75:70:38:eb:b2:ab:b4:97:2c:40:00:50:00:\
    58:00:68:84:80:01:70:05:80:01:00:0a:37:08:01:12:04:75:73:65:72:18:00:22:03:08:ed:03:2a:04:68:\
    64:66:73:32:0a:73:75:70:65:72:67:72:6f:75:70:38:b7:b5:e6:d6:d5:2b:40:00:50:00:58:00:68:82:80:\
    01:70:08:80:01:00:0a:36:08:01:12:03:76:61:72:18:00:22:03:08:ed:03:2a:04:68:64:66:73:32:0a:73:\
    75:70:65:72:67:72:6f:75:70:38:a4:f2:e5:d6:d5:2b:40:00:50:00:58:00:68:85:80:01:70:02:80:01:00:\
    10:00"
    });

    use std::net::ToSocketAddrs;

    let addr = host_port.to_socket_addrs().unwrap().next().unwrap();

    let c = Connection::connect_det(
        &addr,
        vec![1, 2, 3, 4, 4, 3, 2, 1],
        "cloudera".to_owned()
    ).map_err(|e| e.into());

    let src = "/".to_owned();

    let q = pb_cons!(GetListingRequestProto,
        src: src,
        start_after: vec![],
        need_location: false
        );

    let f =
        c.and_then(|conn| conn.send(NnQ::GetListing(q)));

    let c = f.wait().unwrap();

    let (nna_rsp, _c) = c.into_future().wait().unwrap();

    let r = match nna_rsp {
        Some(NnR::GetListing(glr)) => glr,
        _ => panic!("invalid reesp")
    };

    /*for fs in pb_decons!(DirectoryListingProto,
            pb_decons!(GetListingResponseProto, r, dir_list),
            partial_listing)
        {
            let sz = match fs.get_fileType() {
                HdfsFileStatusProto_FileType::IS_DIR => format!("<dir, {} entries>", fs.get_childrenNum()),
                HdfsFileStatusProto_FileType::IS_SYMLINK => format!("->{}", String::from_utf8_lossy(fs.get_symlink())),
                _ => format!("{}", fs.get_length())
            };
            println!("{}\t{}", String::from_utf8_lossy(fs.get_path()), sz);
        }*/
    trace!("RESULT: {:?}", r);

    let x = pb_decons!(DirectoryListingProto,
            pb_decons!(GetListingResponseProto, r, dir_list),
            partial_listing);

    let y: Vec<Cow<str>> = x.iter().map(|fs| String::from_utf8_lossy(fs.get_path())).collect();
    let z: Vec<Cow<str>> =(["benchmarks", "hbase", "solr", "tmp", "user", "var"]).iter().map(|x|Cow::from(*x)).collect();
    assert_eq!(y, z);

    //-----------------------------------
    let _ = t.join().unwrap();
}


//======================================================================================================================
// Old API
//======================================================================================================================



/*
#[derive(Debug)]
pub struct GetListingRequest {
    pub src: String,
    pub start_after: Vec<u8>,
    pub need_location: bool
}

#[derive(Debug)]
pub enum NnaReq {
    GetListing(GetListingRequest)
}
*/

#[derive(Debug)]
pub struct NnaQ {
    inner: NnQ
}

impl NnaQ {
    pub fn new(inner: NnQ) -> NnaQ { NnaQ { inner } }
}


/*
enum FileType {
    IS_DIR,
    IS_FILE,
    IS_SYMLINK
}

pub struct HdfsFileStatus {
    //required FileType fileType = 1;
    file_type: FileType,
    //required bytes path = 2;          // local name of inode encoded java UTF8
    path: Vec<u8>,
    //required uint64 length = 3;
    length: u64,
    //required FsPermissionProto permission = 4;

    //required string owner = 5;
    owner: String,
    //required string group = 6;
    group: String,
    //required uint64 modification_time = 7;

    required uint64 access_time = 8;

    // Optional fields for symlink
    optional bytes symlink = 9;             // if symlink, target encoded java UTF8

    // Optional fields for file
    optional uint32 block_replication = 10 [default = 0]; // only 16bits used
    optional uint64 blocksize = 11 [default = 0];
    optional LocatedBlocksProto locations = 12;  // suppled only if asked by client

    // Optional field for fileId
    optional uint64 fileId = 13 [default = 0]; // default as an invalid id
    optional int32 childrenNum = 14 [default = -1];
    // Optional field for file encryption
    optional FileEncryptionInfoProto fileEncryptionInfo = 15;

    optional uint32 storagePolicy = 16 [default = 0]; // block storage policy id
}

pub struct DirectoryListing {
    //repeated HdfsFileStatusProto partialListing = 1;
    partial_listing: HdfsFileStatus,
    //required uint32 remainingEntries  = 2;
    remaining_entries: u32
}

pub struct GetListingResponse {
    //optional DirectoryListingProto dirList = 1;
    pub dir_list: DirectoryListingProto
}


#[derive(Debug)]
pub enum NnaRsp {
    GetListing(GetListingResponse)
}

impl NnaRsp {
    fn new(nn: NnR) -> NnaRsp {
        match nn {
            NnR::GetListing(glr) => {
                let dir_list = pb_decons!(GetListingResponseProto, glr, dir_list);
                NnaRsp::GetListing(GetListingResponse { dir_list })
            }
        }
    }
}
*/


#[derive(Debug)]
pub struct NnaR {
    pub inner: NnR
}

impl NnaR {
    fn new(inner: NnR) -> NnaR { NnaR { inner } }
}

#[derive(Debug)]
pub struct Connection_ {
    io: Framed<TcpStream, NnCodec>,
    client_id: Vec<u8>,
    call_id: i32
}

impl Connection_ {
    pub fn connect_det(addr: &SocketAddr, client_id: Vec<u8>, eff_user: String) -> BFI<Connection_> {
        trace!("Trying to connect to {}, client_id={:?}, eff_user={}", addr, client_id, eff_user);
        let rv =
            TcpStream::connect(addr)
                .map(|c| Connection_ { io: c.framed(NnCodec::new()), client_id, call_id: 0 })
                .and_then(|c| c.send_handshake(eff_user));
        Box::new(rv)
    }

    pub fn connect(addr: &SocketAddr, eff_user: String) -> BFI<Connection_> {
        Connection_::connect_det(addr, get_client_id(), eff_user)
    }

    fn send_handshake(self, eff_user: String) -> BFI<Connection_> {
        let Connection_ { io, client_id, call_id } = self;
        Box::new(
            io.send(RpcReq::Handshake(HandshakeContext::new(client_id.clone(), eff_user)))
                .map(move |c| Connection_ { io: c, client_id, call_id }))
    }
}

impl Stream for Connection_ {
    type Item = NnaR;
    type Error = Error;
    fn poll(&mut self) -> Result<Async<Option<NnaR>>> {
        match self.io.poll() {
            Ok(Async::NotReady) =>
                Ok(Async::NotReady),
            Ok(Async::Ready(Some(RpcRsp { header, payload }))) =>
                match (validate_response(header, &self.client_id, self.call_id), payload) {
                    (Ok(()), Some(p)) => {
                        self.call_id += 1;
                        Ok(Async::Ready(Some(NnaR::new(p))))
                    }
                    (Ok(()), None) => Err(app_error!(other "Payload required but is empty").into()),
                    (Err(e), _) => Err(e.into())
                }
            Ok(Async::Ready(None)) =>
                Ok(Async::Ready(None)),
            Err(e) =>
                Err(e.into())
        }
    }
}

impl Sink for Connection_ {
    type SinkItem = NnaQ;
    type SinkError = Error;
    fn start_send(&mut self, req: NnaQ) -> Result<AsyncSink<NnaQ>> {
        let method_name = get_method_name(&req.inner);
        match self.io.start_send(RpcReq::Operation(
            OpContext {
                client_id: self.client_id.clone(),
                call_id: self.call_id,
                method_name
            },
            req.inner
        )) {
            Ok(AsyncSink::Ready) => Ok(AsyncSink::Ready),
            Ok(AsyncSink::NotReady(RpcReq::Operation(_, t))) => Ok(AsyncSink::NotReady(NnaQ { inner: t })),
            Err(e) => Err(e.into()),
            _ => Err(app_error!(other "Invalid rejected operation"))
        }
    }

    fn poll_complete(&mut self) -> Result<Async<()>> {
        self.io.poll_complete().map_err(|e| e.into())
    }

    fn close(&mut self) -> Result<Async<()>> {
        self.io.close().map_err(|e| e.into())
    }
}


#[test]
fn test_read_listing_() {
    use util::test::ptk::*;
    let host_port = "127.0.0.1:58120";
    let t = spawn_test_server(host_port, test_script! {

    expect "68:72:70:63:09:00:00:00:00:00:1e:10:08:02:10:00:18:05:22:08:01:02:03:04:04:03:02:01:\
    0c:12:0a:0a:08:63:6c:6f:75:64:65:72:61:00:00:00:58:10:08:02:10:00:18:00:22:08:01:02:03:04:04:\
    03:02:01:3e:0a:0a:67:65:74:4c:69:73:74:69:6e:67:12:2e:6f:72:67:2e:61:70:61:63:68:65:2e:68:61:\
    64:6f:6f:70:2e:68:64:66:73:2e:70:72:6f:74:6f:63:6f:6c:2e:43:6c:69:65:6e:74:50:72:6f:74:6f:63:\
    6f:6c:18:01:07:0a:01:2f:12:00:18:00",

    send "00:00:01:70:12:08:00:10:00:18:09:3a:08:01:02:03:04:04:03:02:01:40:01:db:02:0a:d8:02:0a:\
    3d:08:01:12:0a:62:65:6e:63:68:6d:61:72:6b:73:18:00:22:03:08:ff:03:2a:04:68:64:66:73:32:0a:73:\
    75:70:65:72:67:72:6f:75:70:38:e1:d7:df:d6:d5:2b:40:00:50:00:58:00:68:8e:80:01:70:00:80:01:00:\
    0a:39:08:01:12:05:68:62:61:73:65:18:00:22:03:08:ed:03:2a:05:68:62:61:73:65:32:0a:73:75:70:65:\
    72:67:72:6f:75:70:38:b4:be:e4:95:f7:2b:40:00:50:00:58:00:68:8d:80:01:70:09:80:01:00:0a:31:08:\
    01:12:04:73:6f:6c:72:18:00:22:03:08:ed:03:2a:04:73:6f:6c:72:32:04:73:6f:6c:72:38:e1:91:e8:d6:\
    d5:2b:40:00:50:00:58:00:68:f9:81:01:70:00:80:01:00:0a:36:08:01:12:03:74:6d:70:18:00:22:03:08:\
    ff:07:2a:04:68:64:66:73:32:0a:73:75:70:65:72:67:72:6f:75:70:38:eb:b2:ab:b4:97:2c:40:00:50:00:\
    58:00:68:84:80:01:70:05:80:01:00:0a:37:08:01:12:04:75:73:65:72:18:00:22:03:08:ed:03:2a:04:68:\
    64:66:73:32:0a:73:75:70:65:72:67:72:6f:75:70:38:b7:b5:e6:d6:d5:2b:40:00:50:00:58:00:68:82:80:\
    01:70:08:80:01:00:0a:36:08:01:12:03:76:61:72:18:00:22:03:08:ed:03:2a:04:68:64:66:73:32:0a:73:\
    75:70:65:72:67:72:6f:75:70:38:a4:f2:e5:d6:d5:2b:40:00:50:00:58:00:68:85:80:01:70:02:80:01:00:\
    10:00"
    });

    use std::net::ToSocketAddrs;

    let addr = host_port.to_socket_addrs().unwrap().next().unwrap();

    let c = Connection_::connect_det(
        &addr,
        vec![1, 2, 3, 4, 4, 3, 2, 1],
        "cloudera".to_owned()
    ).map_err(|e| e.into());

    let src = "/".to_owned();

    let q = pb_cons!(GetListingRequestProto,
        src: src,
        start_after: vec![],
        need_location: false
        );

    let f =
        c.and_then(|conn| conn.send(NnaQ { inner: NnQ::GetListing(q) }));

    let c = f.wait().unwrap();

    let (nna_rsp, _c) = c.into_future().wait().unwrap();

    let r = match nna_rsp {
        Some(NnaR { inner: NnR::GetListing(glr) }) => glr,
        _ => panic!("invalid reesp")
    };

    /*for fs in pb_decons!(DirectoryListingProto,
            pb_decons!(GetListingResponseProto, r, dir_list),
            partial_listing)
        {
            let sz = match fs.get_fileType() {
                HdfsFileStatusProto_FileType::IS_DIR => format!("<dir, {} entries>", fs.get_childrenNum()),
                HdfsFileStatusProto_FileType::IS_SYMLINK => format!("->{}", String::from_utf8_lossy(fs.get_symlink())),
                _ => format!("{}", fs.get_length())
            };
            println!("{}\t{}", String::from_utf8_lossy(fs.get_path()), sz);
        }*/
    trace!("RESULT: {:?}", r);

    let x = pb_decons!(DirectoryListingProto,
            pb_decons!(GetListingResponseProto, r, dir_list),
            partial_listing);

    let y: Vec<Cow<str>> = x.iter().map(|fs| String::from_utf8_lossy(fs.get_path())).collect();
    let z: Vec<Cow<str>> =(["benchmarks", "hbase", "solr", "tmp", "user", "var"]).iter().map(|x|Cow::from(*x)).collect();
    assert_eq!(y, z);

    //-----------------------------------
    let _ = t.join().unwrap();
}


//======================================================================================================================
/*
pub struct ConnectionData {
    addr: SocketAddr,
    eff_user: String
}

impl ConnectionData {
    fn connect(&self) -> BFI<Connection> {
        Connection::connect(&self.addr, self.eff_user.clone())
    }
}

pub enum NNCQ {
    SetConnection(ConnectionData),
    ClearConnection,
    Send(NnaQ)
}

pub enum NNCR {
    Recv(NnaR)
}

pub enum Channel {
    Idle,
    Connecting(BFI<Connection>),
    Connected(Connection),
}

impl Sink for Channel {
    type SinkItem = NNCQ;
    type SinkError = Error;

    fn start_send(&mut self, q: NNCQ) -> Result<AsyncSink<NNCQ>> {
        fn start_connecting(cdata: ConnectionData) -> SnV<Channel, Result<AsyncSink<NNCQ>>> {
            let mut f = cdata.connect();
            match f.poll() {
                Ok(Async::Ready(c)) =>
                    SnV::SV(Channel::Connected(c), Ok(AsyncSink::Ready)),
                Ok(Async::NotReady) =>
                    SnV::SV(Channel::Connecting(f), Ok(AsyncSink::Ready)),
                Err(e) =>
                    SnV::V(Err(e.into()))
            }
        }

        match q {
            NNCQ::SetConnection(cdata) => switch_state(
                self,
                |s| match s {
                    Channel::Connected(c) =>
                        match c.close() {
                            Ok(Async::NotReady) =>
                                SnV::V(Ok(AsyncSink::NotReady(NNCQ::SetConnection(cdata)))),
                            Ok(Async::Ready(())) =>
                                start_connecting(cdata),
                            Err(e) =>
                                SnV::V(Err(e))
                        }
                    _ =>
                        start_connecting(cdata)
                }
            ),
            NNCQ::ClearConnection => switch_state(
                self,
                |s| match s {
                    Channel::Connected(c) =>
                        match c.close() {
                            Ok(Async::NotReady) =>
                                SnV::V(Ok(AsyncSink::NotReady(NNCQ::ClearConnection))),
                            Ok(Async::Ready(())) =>
                                SnV::SV(Channel::Idle, Ok(AsyncSink::Ready)),
                            Err(e) =>
                                SnV::V(Err(e))
                        }
                    _ =>
                        SnV::SV(Channel::Idle, Ok(AsyncSink::Ready))
                }
            ),
            NNCQ::Send(q) => switch_state(
                self,
                |s| match s {
                    Channel::Connected(c) =>
                        match c.start_send(q) {
                            Ok(AsyncSink::NotReady(q)) =>
                                SnV::V(Ok(AsyncSink::NotReady(NNCQ::Send(q)))),
                            Ok(AsyncSink::Ready) =>
                                SnV::V(Ok(AsyncSink::Ready)),
                            Err(e) =>
                                SnV::V(Err(e))
                        }
                    Channel::Connecting(f) =>
                        match f.poll() {
                            Ok(Async::Ready(mut c)) =>
                                match c.start_send(q) {
                                    Ok(AsyncSink::NotReady(q)) =>
                                        SnV::SV(Channel::Connected(c), Ok(AsyncSink::NotReady(NNCQ::Send(q)))),
                                    Ok(AsyncSink::Ready) =>
                                        SnV::SV(Channel::Connected(c), Ok(AsyncSink::Ready)),
                                    Err(e) =>
                                        SnV::SV(Channel::Connected(c), Err(e))
                                }
                            Ok(Async::NotReady) =>
                                SnV::V(Ok(AsyncSink::NotReady(NNCQ::Send(q)))),
                            Err(e) =>
                                SnV::V(Err(e.into()))
                        }
                    Channel::Idle =>
                        SnV::V(Err(app_error!(other "Channel: Invalid state (Send without SetConnection)")))
                }
            )
        }
    }

    fn poll_complete(&mut self) -> Result<Async<()>> {
        switch_state(
            self,
            |s| match s {
                Channel::Idle =>
                    SnV::V(Ok(Async::Ready(()))),
                Channel::Connecting(f) =>
                    match f.poll() {
                        Ok(Async::Ready(c)) =>
                            SnV::SV(Channel::Connected(c), Ok(Async::Ready(()))),
                        Ok(Async::NotReady) =>
                            SnV::V(Ok(Async::NotReady)),
                        Err(e) =>
                            SnV::V(Err(e.into()))
                    }
                Channel::Connected(c) =>
                    SnV::V(c.poll_complete())
            }
        )
    }

    fn close(&mut self) -> Result<Async<()>> {
        match self {
            Channel::Connected(c) =>
                c.close(),
            s@ Channel::Connecting(..) => {
                *s = Channel::Idle;
                Ok(Async::Ready(()))
            }
            Channel::Idle =>
                Ok(Async::Ready(()))
        }
    }
}

impl Stream for Channel {
    type Item = NNCR;
    type Error = Error;

    fn poll(&mut self) -> Result<Async<Option<NNCR>>> {
        switch_state(
            self,
            |s| match s {
                Channel::Idle =>
                    SnV::V(Err(app_error!(other "Channel: Invalid state (Recv without SetConnection)"))),
                Channel::Connecting(f) =>
                    match f.poll() {
                        Ok(Async::Ready(mut c)) =>
                            match c.poll() {
                                Ok(Async::Ready(m)) =>
                                    SnV::SV(
                                        Channel::Connected(c),
                                        Ok(Async::Ready(m.map(|m| NNCR::Recv(m))))
                                    ),
                                Ok(Async::NotReady) =>
                                    SnV::SV(Channel::Connected(c), Ok(Async::NotReady)),
                                Err(e) =>
                                    SnV::V(Err(e))
                            }
                        Ok(Async::NotReady) =>
                            SnV::V(Ok(Async::NotReady)),
                        Err(e) =>
                            SnV::V(Err(e.into()))
                    }
                Channel::Connected(c) =>
                    match c.poll() {
                        Ok(Async::Ready(m)) =>
                            SnV::V( Ok(Async::Ready(m.map(|m| NNCR::Recv(m))))),
                        Ok(Async::NotReady) =>
                            SnV::V(Ok(Async::NotReady)),
                        Err(e) =>
                            SnV::V(Err(e))
                    }
            }
        )
    }
}
*/
//--------------------------------------------------------------------------------------------------




use std::io::ErrorKind;
use std::net::SocketAddr;
use tokio_tcp::TcpStream;
//use tokio_core::reactor::Handle;
use tokio_io::codec::Framed;
use tokio_io::AsyncRead;
use futures::{Future, Stream, Poll, Sink};
use futures::future::{ok, err};

use *;
use protobuf_api::*;
use super::codec::{NnCodec, NnQ, NnR, RpcReq, RpcRsp, HandshakeContext, OpContext};

pub enum ProtocolFsmResult {
    Send(NnQ),
    Success,
    Err(IoError)
}

/// Protocol FSM event handler
pub trait ProtocolFsm {
    /// called upon start of operation
    fn start(self) -> (ProtocolFsmResult, Self);
    /// called upon incoming message arrival
    fn incoming(self, NnR) -> (ProtocolFsmResult, Self);
}

/*
/// Protocol FSM event handler (trait object version)
pub trait ProtocolFsmO {
    /// called upon start of operation
    fn start(&self) -> (ProtocolFsmResult, Box<ProtocolFsmO>);
    /// called upon incoming message arrival
    fn incoming(&self, NnR) -> (ProtocolFsmResult, Box<ProtocolFsmO>);
}
*/




pub struct Connection {
    io: Framed<TcpStream, NnCodec>,
    client_id: Vec<u8>,
    call_id: i32
}

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

fn get_client_id() -> Vec<u8> {
    use uuid::Uuid;
    let uuid = Uuid::new_v4();
    Vec::from(&(uuid.as_bytes()[..]))
}

impl Stream for Connection {
    type Item = <Framed<TcpStream, NnCodec> as Stream>::Item;
    type Error = <Framed<TcpStream, NnCodec> as Stream>::Error;

    fn poll(&mut self) -> Poll<Option<Self::Item>, Self::Error> {
        self.io.poll()
    }
}

fn validate_response(response_header: &RpcResponseHeaderProto,
                     client_id: &Vec<u8>, call_id: i32)
    -> Result<()> {
    let (r_call_id, r_status, r_client_id) =
        pb_decons!(RpcResponseHeaderProto, response_header, call_id, status, client_id);

    let _ = match r_status {
        RpcResponseHeaderProto_RpcStatusProto::SUCCESS =>
            Ok(()),
        st @ RpcResponseHeaderProto_RpcStatusProto::ERROR |
        st @ RpcResponseHeaderProto_RpcStatusProto::FATAL
        => {
            let (error_detail, error_msg, exception_class_name) =
                pb_decons!(RpcResponseHeaderProto, response_header, error_detail, error_msg, exception_class_name);
            Err(Error::RPC {
                protocol: "namenode".to_owned(),
                status: st,
                error_detail,
                error_msg: error_msg.to_owned(),
                exception_class_name: exception_class_name.to_owned()
            })
        }
    }?;

    if r_call_id as i32 != call_id {
        Err(app_error!(nn "Call id mismatch, found {}, expected {}", r_call_id, call_id))
    } else if r_client_id != &client_id as &[u8] {
        Err(app_error!(nn "Client id mismatch, found {:?}, expected {:?}", r_client_id, client_id))
    } else{
        Ok(())
    }
}

impl Connection {
    pub fn connect_det(addr: &SocketAddr, client_id: Vec<u8>, eff_user: String) -> BFI<Connection> {
        let rv =
            TcpStream::connect(addr)
                .map(|c| Connection { io: c.framed(NnCodec::new()), client_id, call_id: 0 })
                .and_then(|c| c.send_handshake(eff_user));
        Box::new(rv)
    }

    pub fn connect(addr: &SocketAddr, eff_user: String) -> BFI<Connection> {
        Connection::connect_det(addr, get_client_id(), eff_user)
    }

    #[inline]
    fn broken_pipe_error() -> IoError {
        IoError::new(ErrorKind::BrokenPipe, app_error!(other "broken pipe"))
    }

    fn send_handshake(self, eff_user: String) -> BFI<Connection> {
        let Connection { io, client_id, call_id } = self;
        Box::new(
            io.send(RpcReq::Handshake(HandshakeContext::new(client_id.clone(), eff_user)))
                .map(move |c| Connection { io: c, client_id, call_id }))
    }

    fn send_req(self, q: NnQ, method_name: String) -> BFI<Connection> {
        let Connection { io, client_id, call_id } = self;
        Box::new(
            io.send(
                RpcReq::Operation(
                    OpContext {
                        client_id: client_id.clone(),
                        call_id,
                        method_name,
                    },
                    q
                )
            ).map(
                move |c| Connection { io: c, client_id, call_id: call_id + 1 }
            )
        )
    }

    fn get_resp(self) -> BFI<(NnR, Connection)> {
        let rv =
            self.into_future().and_then(|(orsp, c)|
                match orsp {
                    Some(RpcRsp { header, payload }) =>
                        match (validate_response(&header, &c.client_id, c.call_id - 1), payload) {
                            (Ok(()), Some(p)) => ok((p, c)),
                            (Ok(()), None) => err((app_error!(other "Payload required but is empty").into(), c)),
                            (Err(e), _) => err((e.into(), c))
                        },
                    None => err((Connection::broken_pipe_error(), c))
                }
            ).map_err(|(e, _)| e);
        Box::new(rv)
    }

    #[inline]
    pub fn call(self, q: NnQ) -> BFI<(NnR, Connection)> {
        let method_name = get_method_name(&q);
        Box::new(self.send_req(q, method_name).and_then(|c| c.get_resp()))
    }

    pub fn run<P>(self, p: P) -> BFI<(Connection, P)>
        where P: ProtocolFsm + Send + 'static
    {
        self.fsm_result(p.start())
    }

    fn fsm_result<P>(self, (r, p): (ProtocolFsmResult, P)) -> BFI<(Connection, P)>
        where P: ProtocolFsm + Send + 'static
    {
        match r {
            ProtocolFsmResult::Send(q) =>
                Box::new(self.call(q).and_then(|(r, c)| c.fsm_result(p.incoming(r)))),
            ProtocolFsmResult::Success =>
                Box::new(ok((self, p))),
            ProtocolFsmResult::Err(e) =>
                Box::new(err(e))
        }
    }
}

/// Wraps `call` operation into `ProtocolFsm`
#[derive(Debug)]
pub enum CallW {
    Q(NnQ),
    R(NnR),
    Null
}

impl CallW {
    pub fn new(q: NnQ) -> CallW { CallW::Q(q) }
}

impl ProtocolFsm for CallW {
    fn start(self) -> (ProtocolFsmResult, Self) {
        match self {
            CallW::Q(q) => (ProtocolFsmResult::Send(q), CallW::Null),
            x => (ProtocolFsmResult::Err(app_error!(other "invalid CallWrapper state: expected Q, got `{:?}`", x).into()), CallW::Null)
        }
    }

    fn incoming(self, r: NnR) -> (ProtocolFsmResult, Self) {
        match self {
            CallW::Null => (ProtocolFsmResult::Success, CallW::R(r)),
            x => (ProtocolFsmResult::Err(app_error!(other "invalid CallWrapper state: expected Null, got `{:?}`", x).into()), CallW::Null)
        }
    }
}


fn get_method_name(q: &NnQ) -> String {
    match q {
        &NnQ::GetListing(..) => "getListing".to_owned(),
        &NnQ::GetBlockLocations(..) => "getBlockLocations".to_owned()
    }
}



#[test]
fn test_read_listing() {
    use util::test::ptk::*;
    let t = spawn_test_server("127.0.0.1:58020", test_script! {

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

    let addr = "127.0.0.1:58020".to_socket_addrs().unwrap().next().unwrap();

    let c = Connection::connect_det(
        &addr,
        vec![1, 2, 3, 4, 4, 3, 2, 1],
        "cloudera".to_owned()
    );

    let src = "/".to_owned();

    let q = pb_cons!(GetListingRequestProto,
        src: src,
        start_after: vec![],
        need_location: false
        );

    //let (_, r) = c.call::<_, GetListingResponseProto>(st, "getListing".to_owned(), q).unwrap();
    let f =
        c.and_then(|conn| conn.call(NnQ::GetListing(q)));

    let (nr, _) = f.wait().unwrap();

    let r = match nr {
        NnR::GetListing(glr) => glr,
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
    let x: &[HdfsFileStatusProto] = pb_decons!(DirectoryListingProto,
            pb_decons!(GetListingResponseProto, r, dir_list),
            partial_listing);

    let y: Vec<Cow<str>> = x.iter().map(|fs| String::from_utf8_lossy(fs.get_path())).collect();
    let z: Vec<Cow<str>> =(["benchmarks", "hbase", "solr", "tmp", "user", "var"]).iter().map(|x|Cow::from(*x)).collect();
    assert_eq!(y, z);

    trace!("RESULT: {:?}", r);

    //-----------------------------------
    let _ = t.join().unwrap();
}

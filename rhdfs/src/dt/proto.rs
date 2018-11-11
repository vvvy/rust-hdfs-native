//! Datatransfer protocol upper level

use std::net::SocketAddr;

use futures::prelude::*;

use bytes::Bytes;

use tokio_io::AsyncRead;
use tokio_io::codec::Framed;
use tokio_tcp::TcpStream;

pub use log::Level::Trace as LogTrace;
use super::{
    codec::{DtCodec, DtQ, DtR},
    read_streamer::{ReadStreamer, ReadRange},
    write_streamer::{WriteStreamer},
    checksum::*
};

use protobuf_api::*;
use proto_tools;
use *;


#[derive(Debug, Clone)]
pub struct ExtendedBlock {
    inner: ExtendedBlockProto
}

impl ExtendedBlock {
    #[inline]
    pub fn get_num_bytes(&self) -> u64 { pb_get!(ExtendedBlockProto, self.inner, num_bytes) }
}

impl Into<ExtendedBlockProto> for ExtendedBlock {
    fn into(self) -> ExtendedBlockProto {
        self.inner
    }
}

impl From<ExtendedBlockProto> for ExtendedBlock {
    fn from(inner: ExtendedBlockProto) -> Self {
        ExtendedBlock { inner }
    }
}


#[derive(Debug, Clone)]
pub struct Token {
    inner: Option<TokenProto>
}

impl Token {
    fn empty() -> Token { Token {
        inner: None
        //pb_cons!(TokenProto, identifier: vec![], password: vec![], kind: "".to_string(), service: "".to_string())
    } }
}

impl Into<Option<TokenProto>> for Token {
    fn into(self) -> Option<TokenProto> {
        self.inner
    }
}

impl From<TokenProto> for Token {
    fn from(inner: TokenProto) -> Self {
        Token { inner: Some(inner) }
    }
}


#[derive(Debug)]
pub struct ReadBlock {
    pub b: ExtendedBlock,
    pub t: Token,
    pub offset: u64,
    pub len: u64
}

fn mk_read_block(rb: ReadBlock, client_name: &str) -> OpReadBlockProto {
    let bh = match rb.t.into() {
        Some(t) => pb_cons!(BaseHeaderProto, block: rb.b.into(), token: t),
        None => pb_cons!(BaseHeaderProto, block: rb.b.into())
    };

    pb_cons!(OpReadBlockProto,
           header: pb_cons!(ClientOperationHeaderProto,
                client_name: client_name.to_owned(),
                base_header: bh
           ),
           offset: rb.offset,
           len: rb.len
       )
}


#[derive(Debug)]
pub struct WriteBlock {
    pub b: ExtendedBlock,
    pub t: Token,
    pub block_len: usize,
    pub packet_len: usize,
    pub targets: Vec<DatanodeInfoProto>,
    pub storage_types: Vec<StorageTypeProto>
}

fn mk_write_block(wb: WriteBlock, client_name: &str) -> (OpWriteBlockProto, WriteStreamer) {
    let bh = match wb.t.into() {
        Some(t) => pb_cons!(BaseHeaderProto, block: wb.b.into(), token: t),
        None => pb_cons!(BaseHeaderProto, block: wb.b.into())
    };
    let tlen = wb.targets.len() as u32;
    let st = wb.storage_types[0];

    //DFS_BYTES_PER_CHECKSUM_KEY = "dfs.bytes-per-checksum"
    //DFS_BYTES_PER_CHECKSUM_DEFAULT = 512
    //DFS_CHECKSUM_TYPE_KEY = "dfs.checksum.type"
    //DFS_CHECKSUM_TYPE_DEFAULT = "CRC32C"
    let checksum_type = ChecksumTypeProto::CHECKSUM_CRC32C;
    let bytes_per_checksum = 512usize;


    let owbp = pb_cons!(OpWriteBlockProto,
            header: pb_cons!(ClientOperationHeaderProto,
                client_name: client_name.to_owned(),
                base_header: bh
            ),
            targets: wb.targets,
            stage: OpWriteBlockProto_BlockConstructionStage::PIPELINE_SETUP_CREATE,
            pipeline_size: tlen,
            min_bytes_rcvd: 0,
            max_bytes_rcvd: 0,
            latest_generation_stamp: 0,
            requested_checksum: pb_cons!(ChecksumProto,
                type: checksum_type,
                bytes_per_checksum: bytes_per_checksum as u32),

            storage_type: st,
            target_storage_types: wb.storage_types

            //caching_strategy,
            //allow_lazy_persist
        );

    let streamer = WriteStreamer::new(
        checksum_from_args(checksum_type, bytes_per_checksum),
        wb.block_len,
        wb.packet_len
    );

    (owbp, streamer)




    /*
    pb_cons!(OpReadBlockProto,
           header: pb_cons!(ClientOperationHeaderProto,
                client_name: client_name.to_owned(),
                base_header: bh
           ),
           offset: rb.offset,
           len: rb.len
       )
       */
    /*
    pub fn new(h: BaseHeaderProto,
               targets: Vec<DatanodeInfoProto>,
               storage_types: Vec<StorageTypeProto>,
               r: R) -> Self {
        let tlen = targets.len() as u32;
        let st = storage_types[0];

        //DFS_BYTES_PER_CHECKSUM_KEY = "dfs.bytes-per-checksum"
        //DFS_BYTES_PER_CHECKSUM_DEFAULT = 512
        //DFS_CHECKSUM_TYPE_KEY = "dfs.checksum.type"
        //DFS_CHECKSUM_TYPE_DEFAULT = "CRC32C"

        let owbp = pb_cons!(OpWriteBlockProto,
            header: pb_cons!(ClientOperationHeaderProto,
                //client_name: client_name.to_owned(),
                base_header: h
            ),
            targets: targets,
            stage: OpWriteBlockProto_BlockConstructionStage::PIPELINE_SETUP_CREATE,
            pipeline_size: tlen,
            min_bytes_rcvd: 0,
            max_bytes_rcvd: 0,
            latest_generation_stamp: 0,
            requested_checksum: pb_cons!(ChecksumProto,
                type: ChecksumTypeProto::CHECKSUM_CRC32C,
                bytes_per_checksum: 512),

            storage_type: st,
            target_storage_types: storage_types

            //caching_strategy,
            //allow_lazy_persist
        );
        WriteBlock::Init(owbp, BlockWriteState::new(r, 0 as i64))
    }
    pub fn state(self) -> BlockWriteState<R> {
        match self {
            WriteBlock::Init(_, bws) |
            WriteBlock::ResponseWait(bws, _) |
            WriteBlock::End(bws) =>
                bws,
            WriteBlock::Packet(bwt) =>
                bwt.decons()
        }
    }

    */
}

fn is_block_op_response_success(borp: &BlockOpResponseProto) -> bool {
    pb_decons!(BlockOpResponseProto, borp, status) == DtStatus::SUCCESS
}


#[derive(Debug)]
pub enum DtaQ {
    ReadBlock(ReadBlock),
    WriteBlock(WriteBlock),
    Data(Bytes)
}

#[derive(Debug)]
pub enum DtaR {
    Data(Bytes)
}

//----------------------------------------------------------------

type Action = proto_tools::Action<DtQ, DtaR>;
type NetEvent = proto_tools::NetEvent<DtR>;
type UserEvent = proto_tools::UserEvent<DtaQ>;

#[derive(Debug)]
enum SendingState {
    Idle,
    Sending
}

#[derive(Debug)]
enum ConnectionState {
    Idle,
    RbSendReq(ReadRange),
    RbRecvInitResp(ReadRange),
    RbStreaming(ReadStreamer),
    RbSendFinalAck,
    WbSendReq(WriteStreamer),
    WbRecvInitResp(WriteStreamer),
    WbStreaming(WriteStreamer, SendingState)
}

impl ConnectionState {
    #[inline]
    fn fc_bits(a: Action, s: ConnectionState) -> (Action, ConnectionState) {
        use self::ConnectionState as S;
        (a.bits(match &s {
            S::Idle => proto_bits!(-recv, +accept, +send_complete),
            S::RbRecvInitResp(..) |
            S::RbStreaming(..) => proto_bits!(+recv, -accept, -send_complete),
            S::WbSendReq(..) => proto_bits!(-recv, -accept, -send_complete),
            S::WbRecvInitResp(..) => proto_bits!(+recv, -accept, -send_complete),
            S::WbStreaming(ws, ..) => if ws.push_paused() {
                proto_bits!(+recv, -accept, -send_complete)
            } else {
                proto_bits!(+recv, +accept, +send_complete)
            }
            _ => proto_bits!(-recv, -accept, +send_complete)
        }),
        s)
    }
}

struct ProtoFsm {
    state: ConnectionState,
    client_name: String
}

macro_rules! fsmc {
    { send($v:expr) / $s:expr } => { ConnectionState::fc_bits(Action::z().send($v), $s) };
    { err($v:expr) / $s:expr } => { ConnectionState::fc_bits(Action::z().err($v), $s) };
    { err($v:expr) send($w:expr) / $s:expr } => { ConnectionState::fc_bits(Action::z().err($v).send($w), $s) };
    { deliver($v:expr) / $s:expr } => { ConnectionState::fc_bits(Action::z().deliver($v), $s) };
    { deliver($v:expr) send($w:expr) / $s:expr } => { ConnectionState::fc_bits(Action::z().deliver($v).send($w), $s) };
    { / $s:expr } => { ConnectionState::fc_bits(Action::z(), $s) };
}

impl ProtoFsm {
    fn new(client_name: String) -> ProtoFsm { ProtoFsm { state: ConnectionState::Idle, client_name } }
}

impl proto_tools::ProtoFsm for ProtoFsm {
    type NQ = DtQ;
    type NR = DtR;
    type UQ = DtaQ;
    type UR = DtaR;

    fn handle_n(&mut self, ne: NetEvent) -> Action {
        use self::ConnectionState as S;
        use proto_tools::NetEvent as E;
        trace!("handle_n[{}]: {:?}/{:?} => ..", self.client_name, self.state, ne);
        let (a, s) = match (std::mem::replace(&mut self.state, ConnectionState::Idle), ne) {
            (S::Idle, E::Init) =>
                fsmc!(/ S::Idle),
            (S::RbSendReq(rr), E::Idle) =>
                fsmc!(/ S::RbRecvInitResp(rr)),
            (S::RbRecvInitResp(rr), E::Incoming(DtR::ReadBlockResponse(borp))) =>
                if is_block_op_response_success(&borp) {
                    let roci =
                        pb_decons!(BlockOpResponseProto, borp, read_op_checksum_info);
                    let (checksum, _chunk_offset) =
                        pb_decons!(ReadOpChecksumInfoProto, roci, checksum, chunk_offset);
                    fsmc!(/S::RbStreaming(ReadStreamer::new(rr, checksum_from_proto(checksum))))
                } else {
                    let (s, m) = pb_decons!(BlockOpResponseProto, borp, status, message);
                    fsmc!(err(app_error!(dt s, m)) / S::Idle)
                }
            (S::RbStreaming(mut rs), E::Incoming(DtR::Packet(packet))) => {
                let is_last = packet.is_last();
                let pp = rs.process_packet(packet);
                let client_status =  if let Err(Error::DataTransfer(dts, _)) = &pp {
                    Some(*dts)
                } else if is_last {
                    Some(rs.get_success_status())
                } else {
                    None
                };
                if let Some(cs) = client_status {
                    let crs = DtQ::ClientReadStatus(pb_cons!(ClientReadStatusProto, status: cs));
                    match pp {
                        Ok(bs) =>
                            if bs.is_empty() {
                                fsmc!(deliver(DtaR::Data(bs)) send(crs) / S::RbSendFinalAck)
                            } else {
                                fsmc!(err(app_error!(other "dt protocol error: last packet is not empty")) / S::Idle)
                            }

                        Err(e) => fsmc!(err(e) send(crs) / S::Idle)
                    }
                } else {
                    //suppress empty Data
                    match pp {
                        Ok(bs) =>
                            if !bs.is_empty() {
                                fsmc!(deliver(DtaR::Data(bs)) / S::RbStreaming(rs))
                            } else {
                                fsmc!(/ S::RbStreaming(rs))
                            }
                        Err(e) =>
                            fsmc!(err(e) / S::Idle)
                    }
                }
            }
            (S::RbSendFinalAck, E::Idle) =>
                fsmc!(/ S::Idle),
            (S::WbSendReq(ws), E::Idle) =>
                fsmc!(/ S::WbRecvInitResp(ws)),
            (S::WbRecvInitResp(ws), E::Incoming(DtR::WriteBlockResponse(borp))) =>
                if is_block_op_response_success(&borp) {
                    //fsmc!(/S::WbStreaming(ws))
                    unimplemented!()
                } else {
                    let (s, m) = pb_decons!(BlockOpResponseProto, borp, status, message);
                    fsmc!(err(app_error!(dt s, m)) / S::Idle)
                }
            (S::WbStreaming(ws, ss), E::Incoming(DtR::Ack(ack))) =>
               unimplemented!(),
            (s, e) =>
                fsmc!(err(app_error!(other "Invalid s/ne {:?}/{:?}", s, e)) / S::Idle)
        };
        trace!("handle_n[{}]: .. => {:?}/{:?}", self.client_name, a, s);
        self.state = s;
        a
    }
    fn handle_u(&mut self, ue: UserEvent) -> Action {
        use self::ConnectionState as S;
        use proto_tools::UserEvent as E;
        trace!("handle_u[{}]: event {:?}", self.client_name, ue);
        let (a, s) = match (std::mem::replace(&mut self.state, ConnectionState::Idle), ue) {
            (S::Idle, E::Message(DtaQ::ReadBlock(rb))) => {
                let transfer_range = (rb.offset as i64, (rb.offset + rb.len) as i64);
                fsmc!(send(DtQ::ReadBlock(mk_read_block(rb, &self.client_name))) / S::RbSendReq(transfer_range))
            }
            (S::Idle, E::Message(DtaQ::WriteBlock(wb))) => {
                let (owbp, ws) = mk_write_block(wb, &self.client_name);
                fsmc!(send(DtQ::WriteBlock(owbp)) / S::WbSendReq(ws))
            }
            (S::WbStreaming(ws, ss), E::Message(DtaQ::Data(bs))) => {
                unimplemented!()
            }
            (s, E::Flush) =>
                fsmc!(/ s),
            (s, e) =>
                fsmc!(err(app_error!(other "Invalid s/ue {:?}/{:?}", s, e)) / S::Idle)
        };
        trace!("handle_u[{}]: {:?}/.. => {:?}/{:?}", self.client_name, self.state, a, s);
        self.state = s;
        a
    }
}

pub struct Connection_ {
    inner: proto_tools::layer::T<Framed<TcpStream, DtCodec>, ProtoFsm>
}

impl Connection_ {
    pub fn connect(addr: &SocketAddr, client_name: String) -> BFI<Connection_> {
        trace!("Trying to connect to {}, client_name={}", addr, client_name);
        let rv =
            TcpStream::connect(addr)
                .map(|c| Connection_ {
                    inner: proto_tools::layer::new(
                        c.framed(DtCodec::new()),
                        ProtoFsm::new(client_name)
                    )
                });
        Box::new(rv)
    }

}

impl Stream for Connection_ {
    type Item = DtaR;
    type Error = Error;
    fn poll(&mut self) -> Result<Async<Option<DtaR>>> {
        self.inner.poll()
    }
}

impl Sink for Connection_ {
    type SinkItem = DtaQ;
    type SinkError = Error;
    fn start_send(&mut self, req: DtaQ) -> Result<AsyncSink<DtaQ>> {
        self.inner.start_send(req)
    }

    fn poll_complete(&mut self) -> Result<Async<()>> {
        self.inner.poll_complete()
    }

    fn close(&mut self) -> Result<Async<()>> {
        self.inner.close()
    }
}

#[test]
fn test_old_read_block_simple_() {
    use std::io::{Read, ErrorKind};

    pub struct AsyncReadBlock {
        c: Connection_,
        b: Bytes
    }

    impl AsyncReadBlock {
        pub fn new(c: Connection_) -> AsyncReadBlock { AsyncReadBlock { c, b: Bytes::new() } }
        pub fn decons(self) -> (Connection_, Bytes) { (self.c, self.b) }
        fn fill_buffer(&mut self, buf: &mut [u8]) -> usize {
            if self.b.is_empty() {
                0
            } else {
                let l = self.b.len().min(buf.len());
                let b = self.b.split_to(l);
                buf[0..l].copy_from_slice(&b[..]);
                l
            }
        }
    }

    impl Read for AsyncReadBlock {
        fn read(&mut self, buf: &mut [u8]) -> IoResult<usize> {
            let l = self.fill_buffer(buf);
            if l > 0 {
                Ok(l)
            } else {
                match self.c.poll() {
                    Ok(Async::Ready(Some(DtaR::Data(b)))) => {
                        self.b = b;
                        Ok(self.fill_buffer(buf))   //if b is empty, it's over
                    }
                    Ok(Async::Ready(None)) =>
                        Err(IoError::new(ErrorKind::BrokenPipe, app_error!(other "Premature end of input on ReadBlock"))),
                    Ok(Async::NotReady) =>
                        Err(ErrorKind::WouldBlock.into()),
                    Err(e) =>
                        Err(e.into())
                }
            }
        }
    }

    impl AsyncRead for AsyncReadBlock { }


    use util::test::ptk::*;
    let host_port = "127.0.0.1:60110";

    use std::io::{Cursor};

    init_env_logger!();

    let t = spawn_test_server(host_port, test_script! {
       //E 00:1c:51:41:0a:3b:0a:34:0a:32:0a:25:42:50:2d:31:39:31:34:38:35:33:32:34:33:2d:31:32:37:2e:30:2e:30:2e:31:2d:31:35:30:30:34:36:37:36:30:37:30:35:32:10:f3:87:80:80:04:18:df:0f:20:05:12:03:61:62:63:10:00:18:05
       expect "\
       00:1c:51:41:0a:3b:0a:34:0a:32:0a:25:42:50:2d:31:39:31:34:38:35:33:32:34:33:2d:31:32:37:\
       2e:30:2e:30:2e:31:2d:31:35:30:30:34:36:37:36:30:37:30:35:32:10:f3:87:80:80:04:18:df:0f:\
       20:05:12:03:61:62:63:10:00:18:05",

       //S 0d:08:00:22:09:0a:05:08:02:10:80:04:10:00
       send "0d:08:00:22:09:0a:05:08:02:10:80:04:10:00",

       //S 00:00:00:0d:00:19:09:00:00:00:00:00:00:00:00:11:00:00:00:00:00:00:00:00:18:00:25:05:00:00:00:a9:c7:c0:1b
       send "\
       00:00:00:0d:00:19:09:00:00:00:00:00:00:00:00:11:00:00:00:00:00:00:00:00:18:00:25:05:00:\
       00:00:a9:c7:c0:1b",

       //S 41:42:43:44:0a
       send "41:42:43:44:0a",

       //S 00:00:00:04:00:19:09:05:00:00:00:00:00:00:00:11:01:00:00:00:00:00:00:00:18:01:25:00:00:00:00
       send "\
       00:00:00:04:00:19:09:05:00:00:00:00:00:00:00:11:01:00:00:00:00:00:00:00:18:01:25:00:00:\
       00:00",

       //E 02:08:06
       expect "02 08 06"
    });

    use std::net::ToSocketAddrs;
    use std::borrow::Cow;
    use futures::Future;

    let addr = host_port.to_socket_addrs().unwrap().next().ok_or(app_error!(other "DN host not found")).unwrap();
    let conn_f = Connection_::connect(&addr, "abc".to_owned());

    let rb = ReadBlock {
        b: ExtendedBlock { inner: pb_cons!(ExtendedBlockProto,
                       pool_id: "BP-1914853243-127.0.0.1-1500467607052".to_owned(),
                       block_id: 1073742835,
                       generation_stamp: 2015,
                       num_bytes: 5
                   ) },
        t: Token::empty(),
        offset: 0,
        len: 5
    };

    let (cnt, _arb, cv) = conn_f
        .and_then(|conn| conn.send(DtaQ::ReadBlock(rb)).map_err(|e| e.into()))
        .and_then(|conn| tokio_io::io::copy(
            AsyncReadBlock::new(conn),
            Cursor::new(Vec::new())
        ))
        .wait()
        .unwrap();

    assert_eq!(cnt, 5);
    let w = cv.into_inner();
    assert_eq!(w, vec![65, 66, 67, 68, 10]);
    //-----------------------------------
    let _ = t.join().unwrap();
}

//======================================================================================================================
// new API
//======================================================================================================================

use proto_tools::*;

pub struct Connection {
    io: Framed<TcpStream, DtCodec>
}

impl Connection {
    pub fn connect(addr: &SocketAddr) -> BFI<Connection> {
        trace!("Trying to connect to {}", addr);
        let rv =
            TcpStream::connect(addr)
                .map(|c| Connection {
                    io: c.framed(DtCodec::new())
                });
        Box::new(rv)
    }

}

impl Stream for Connection {
    type Item = DtR;
    type Error = Error;
    fn poll(&mut self) -> Result<Async<Option<DtR>>> {
        self.io.poll()
    }
}

impl Sink for Connection {
    type SinkItem = DtQ;
    type SinkError = Error;
    fn start_send(&mut self, req: DtQ) -> Result<AsyncSink<DtQ>> {
        self.io.start_send(req)
    }

    fn poll_complete(&mut self) -> Result<Async<()>> {
        self.io.poll_complete()
    }

    fn close(&mut self) -> Result<Async<()>> {
        self.io.close()
    }
}



#[derive(Debug)]
enum ReadBlockSend {
    OpReadBlock(ReadBlock),
    ClientReadStatus(ClientReadStatusProto)
}

#[derive(Debug)]
enum ReadBlockRecv {
    OpReadBlockResponse(ReadRange),
    Data(ReadStreamer)
}

#[derive(Debug)]
pub struct ReadBlockFsm {
    send: Option<ReadBlockSend>,
    recv: Option<ReadBlockRecv>,
    client_name: String
}

impl ReadBlockFsm {
    pub fn new(client_name: String, rb: ReadBlock) -> ReadBlockFsm {
        ReadBlockFsm {
            send: Some(ReadBlockSend::OpReadBlock(rb)),
            recv: None,
            client_name
        }
    }
}

impl StreamProtocolFsm for ReadBlockFsm {
    type R = Bytes;
    type LQ = DtQ;
    type LR = DtR;

    fn get_downstream(&mut self) -> Result<SendReq<DtQ>> {
        match self.send.take() {
            Some(ReadBlockSend::OpReadBlock(rb)) => {
                self.recv = Some(ReadBlockRecv::OpReadBlockResponse((rb.offset as i64, (rb.offset + rb.len) as i64)));
                Ok(SendReq::EnqueueAndFlush(DtQ::ReadBlock(mk_read_block(rb, &self.client_name))))
            }
            Some(ReadBlockSend::ClientReadStatus(crs)) =>
                Ok(SendReq::EnqueueAndFlush(DtQ::ClientReadStatus(crs))),
            None =>
                Ok(SendReq::NOP)
        }
    }

    fn handle_upstream(&mut self, lr: Option<DtR>) -> Result<Async<Option<Bytes>>> {
        if let Some(DtR::ReadBlockResponse(borp)) = lr {
            if let Some(ReadBlockRecv::OpReadBlockResponse(rr)) = self.recv.take() {
                if is_block_op_response_success(&borp) {
                    let roci =
                        pb_decons!(BlockOpResponseProto, borp, read_op_checksum_info);
                    let (checksum, _chunk_offset) =
                        pb_decons!(ReadOpChecksumInfoProto, roci, checksum, chunk_offset);
                    self.recv = Some(ReadBlockRecv::Data(ReadStreamer::new(rr, checksum_from_proto(checksum))));
                    Ok(Async::NotReady)
                } else {
                    let (s, m) = pb_decons!(BlockOpResponseProto, borp, status, message);
                    Err(app_error!(dt s, m))
                }
            } else {
                Err(app_error!(other "ReadBlockResponse: invalid state {:?}", self))
            }
        } else if let Some(DtR::Packet(packet)) = lr {
            if let Some(ReadBlockRecv::Data(mut rs)) = self.recv.take() {
                let is_last = packet.is_last();
                let pp = rs.process_packet(packet);
                let client_status = if let Err(Error::DataTransfer(dts, _)) = &pp {
                    Some(*dts)
                } else if is_last {
                    Some(rs.get_success_status())
                } else {
                    None
                };
                let rv = if let Some(cs) = client_status {
                    self.send = Some(ReadBlockSend::ClientReadStatus(pb_cons!(ClientReadStatusProto, status: cs)));
                    match pp {
                        Ok(bs) =>
                            if bs.is_empty() {
                                //previously was
                                //Ok(Async::Ready(Some(bs)))
                                Ok(Async::Ready(None))
                            } else {
                                Err(app_error!(other "dt protocol error: last packet is not empty"))
                            }
                        Err(e) =>
                            Err(e)
                    }
                } else {
                    self.recv = Some(ReadBlockRecv::Data(rs));
                    //suppress empty Data
                    match pp {
                        Ok(bs) =>
                            if !bs.is_empty() {
                                Ok(Async::Ready(Some(bs)))
                            } else {
                                Ok(Async::NotReady)
                            }
                        Err(e) =>
                            Err(e)
                    }
                };
                rv
            } else {
                Err(app_error!(other "Packet: invalid state {:?}", self))
            }
        } else if lr.is_none() {
            Err(app_error!(other "premature EOS"))
        } else {
            Err(app_error!(other "Unexpected message {:?} in ReadBlock", lr))
        }
    }
}

type ReadBlockConnection = StreamProtocol<Connection, ReadBlockFsm>;

pub struct ReadBlockConnectionData {
    pub addr: SocketAddr,
    pub client_name: String,
    pub read_block: ReadBlock
}

impl Into<BFI<ReadBlockConnection>> for ReadBlockConnectionData {
    fn into(self) -> BFI<ReadBlockConnection> {
        Box::new(
            Connection::connect(&self.addr, ).map(move |c|
                proto_tools::StreamProtocol::new(c, ReadBlockFsm::new(self.client_name, self.read_block))
            )
        )
    }
}

pub struct ReadBlockChannel {
    inner: HChannel<Bytes, ReadBlockConnection, ReadBlockConnectionData>
}

impl ReadBlockChannel {
    pub fn new() -> ReadBlockChannel {
        ReadBlockChannel { inner: HChannel::new() }
    }
    pub fn connect(cdata: ReadBlockConnectionData) -> ReadBlockChannel {
        ReadBlockChannel { inner: HChannel::connect(cdata) }
    }
}

impl Stream for ReadBlockChannel {
    type Item = Bytes;
    type Error = Error;

    fn poll(&mut self) -> Result<Async<Option<Bytes>>> {
        self.inner.poll()
    }
}

impl Sink for ReadBlockChannel {
    type SinkItem = HCQ<ReadBlockConnectionData>;
    type SinkError = Error;

    fn start_send(&mut self, item: HCQ<ReadBlockConnectionData>) -> Result<AsyncSink<HCQ<ReadBlockConnectionData>>> {
        self.inner.start_send(item)
    }

    fn poll_complete(&mut self) -> Result<Async<()>> {
        self.inner.poll_complete()
    }

    fn close(&mut self) -> Result<Async<()>> {
        self.inner.close()
    }
}

//======================================================================================================================

#[test]
fn test_read_block_simple() {
    use std::io::{Read, ErrorKind};

    pub struct AsyncReadBlock {
        c: ReadBlockConnection,
        b: Bytes
    }

    impl AsyncReadBlock {
        pub fn new(c: ReadBlockConnection) -> AsyncReadBlock { AsyncReadBlock { c, b: Bytes::new() } }
        pub fn decons(self) -> (ReadBlockConnection, Bytes) { (self.c, self.b) }
        fn fill_buffer(&mut self, buf: &mut [u8]) -> usize {
            if self.b.is_empty() {
                0
            } else {
                let l = self.b.len().min(buf.len());
                let b = self.b.split_to(l);
                buf[0..l].copy_from_slice(&b[..]);
                l
            }
        }
    }

    impl Read for AsyncReadBlock {
        fn read(&mut self, buf: &mut [u8]) -> IoResult<usize> {
            trace!("***READ***");
            let l = self.fill_buffer(buf);
            if l > 0 {
                Ok(l)
            } else {
                match self.c.poll() {
                    Ok(Async::Ready(Some(b))) => {
                        self.b = b;
                        Ok(self.fill_buffer(buf))
                    }
                    Ok(Async::Ready(None)) => {
                        //previously was
                        //Err(IoError::new(ErrorKind::BrokenPipe, app_error!(other "Premature end of input on ReadBlock"))),
                        self.b = Bytes::new();
                        Ok(self.fill_buffer(buf))
                    }
                    Ok(Async::NotReady) =>
                        Err(ErrorKind::WouldBlock.into()),
                    Err(e) =>
                        Err(e.into())
                }
            }
        }
    }

    impl AsyncRead for AsyncReadBlock { }


    use util::test::ptk::*;
    let host_port = "127.0.0.1:60010";

    use std::io::{Cursor};

    init_env_logger!();

    let t = spawn_test_server(host_port, test_script! {
       //E 00:1c:51:41:0a:3b:0a:34:0a:32:0a:25:42:50:2d:31:39:31:34:38:35:33:32:34:33:2d:31:32:37:2e:30:2e:30:2e:31:2d:31:35:30:30:34:36:37:36:30:37:30:35:32:10:f3:87:80:80:04:18:df:0f:20:05:12:03:61:62:63:10:00:18:05
       expect "\
       00:1c:51:41:0a:3b:0a:34:0a:32:0a:25:42:50:2d:31:39:31:34:38:35:33:32:34:33:2d:31:32:37:\
       2e:30:2e:30:2e:31:2d:31:35:30:30:34:36:37:36:30:37:30:35:32:10:f3:87:80:80:04:18:df:0f:\
       20:05:12:03:61:62:63:10:00:18:05",

       //S 0d:08:00:22:09:0a:05:08:02:10:80:04:10:00
       send "0d:08:00:22:09:0a:05:08:02:10:80:04:10:00",

       //S 00:00:00:0d:00:19:09:00:00:00:00:00:00:00:00:11:00:00:00:00:00:00:00:00:18:00:25:05:00:00:00:a9:c7:c0:1b
       send "\
       00:00:00:0d:00:19:09:00:00:00:00:00:00:00:00:11:00:00:00:00:00:00:00:00:18:00:25:05:00:\
       00:00:a9:c7:c0:1b",

       //S 41:42:43:44:0a
       send "41:42:43:44:0a",

       //S 00:00:00:04:00:19:09:05:00:00:00:00:00:00:00:11:01:00:00:00:00:00:00:00:18:01:25:00:00:00:00
       send "\
       00:00:00:04:00:19:09:05:00:00:00:00:00:00:00:11:01:00:00:00:00:00:00:00:18:01:25:00:00:\
       00:00",

       //E 02:08:06
       expect "02 08 06"
    });

    use std::net::ToSocketAddrs;
    use std::borrow::Cow;
    use futures::Future;

    let addr = host_port.to_socket_addrs().unwrap().next().ok_or(app_error!(other "DN host not found")).unwrap();
    let client_name = "abc".to_owned();

    let read_block = ReadBlock {
        b: ExtendedBlock { inner: pb_cons!(ExtendedBlockProto,
                       pool_id: "BP-1914853243-127.0.0.1-1500467607052".to_owned(),
                       block_id: 1073742835,
                       generation_stamp: 2015,
                       num_bytes: 5
                   ) },
        t: Token::empty(),
        offset: 0,
        len: 5
    };

    let cdata = ReadBlockConnectionData { addr, client_name, read_block };
    let conn: BFI<ReadBlockConnection> = cdata.into();

    let (cnt, _arb, cv) = conn
        .and_then(|conn| tokio_io::io::copy(
            AsyncReadBlock::new(conn),
            Cursor::new(Vec::new())
        ))
        .wait()
        .unwrap();

    assert_eq!(cnt, 5);
    let w = cv.into_inner();
    assert_eq!(w, vec![65, 66, 67, 68, 10]);
    //-----------------------------------
    let _ = t.join().unwrap();
}

/*
#[test]
fn test_write_block_simple() {
    use util::test::ptk::*;
    let host_port = "127.0.0.1:61010";

    use std::io::{Cursor};

    init_env_logger!();
}
*/
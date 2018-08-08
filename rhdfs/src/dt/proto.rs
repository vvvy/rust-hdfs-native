//! Datatransfer protocol upper level

use std::net::SocketAddr;

use futures::prelude::*;

use bytes::Bytes;

use tokio_io::AsyncRead;
use tokio_io::codec::Framed;
use tokio_tcp::TcpStream;

pub use log::Level::Trace as LogTrace;
use super::{
    codec::{DtCodec, DtQ, DtR, OpBlockReadMessage},
    packet::BlockDataPacket,
    checksum::{ChecksumValidator, new_checksum}
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

#[inline]
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
pub enum DtaQ {
    ReadBlock(ReadBlock),
    //WriteBlock(WriteBlock),
    //Data(Bytes)
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
enum ConnectionState {
    Idle,
    RbSendReq(ReadRange),
    RbRecvInitResp(ReadRange),
    RbStreaming(ReadStreamer),
    RbSendFinalAck
}

impl ConnectionState {
    #[inline]
    fn fc_bits(a: Action, s: ConnectionState) -> (Action, ConnectionState) {
        use self::ConnectionState as S;
        (match &s {
            S::Idle => proto_bits!(a, -recv, +accept, +send_complete),  //.send_complete().accept(),
            S::RbRecvInitResp(..) | S::RbStreaming(..) => proto_bits!(a, +recv, -accept, +send_complete), //a.recv(),
            _ => proto_bits!(a, -recv, -accept, +send_complete)
        },
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
            (S::RbRecvInitResp(rr), E::Incoming(DtR::ReadBlock(OpBlockReadMessage::Initial(has_data, borp)))) =>
                if has_data {
                    let roci =
                        pb_decons!(BlockOpResponseProto, borp, read_op_checksum_info);
                    let (checksum, _chunk_offset) =
                        pb_decons!(ReadOpChecksumInfoProto, roci, checksum, chunk_offset);
                    fsmc!(/S::RbStreaming(ReadStreamer::new(rr, new_checksum(checksum))))
                } else {
                    let (s, m) = pb_decons!(BlockOpResponseProto, borp, status, message);
                    fsmc!(err(app_error!(dt s, m)) / S::Idle)
                }
            (S::RbStreaming(mut rs), E::Incoming(DtR::ReadBlock(OpBlockReadMessage::Packet(packet, is_last)))) => {
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


pub struct Connection {
    inner: proto_tools::layer::T<Framed<TcpStream, DtCodec>, ProtoFsm>
}

impl Connection {
    pub fn connect(addr: &SocketAddr, client_name: String) -> BFI<Connection> {
        trace!("Trying to connect to {}, client_name={}", addr, client_name);
        let rv =
            TcpStream::connect(addr)
                .map(|c| Connection {
                    inner: proto_tools::layer::new(
                        c.framed(DtCodec::new()),
                        ProtoFsm::new(client_name)
                    )
                });
        Box::new(rv)
    }

}

impl Stream for Connection {
    type Item = DtaR;
    type Error = Error;
    fn poll(&mut self) -> Result<Async<Option<DtaR>>> {
        self.inner.poll()
    }
}

impl Sink for Connection {
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

//--------------------------------------------------------------------------------------------------

type ReadRange = (i64, i64);

#[derive(Debug)]
struct ReadStreamer {
    checksum: Box<ChecksumValidator>,
    read_range: ReadRange,
    seqno: i64,
    offset: i64
}

impl ReadStreamer {
    fn new(read_range: ReadRange, checksum: Box<ChecksumValidator>) -> ReadStreamer {
        ReadStreamer {
            checksum,
            read_range,
            seqno: 0,
            offset: 0,
        }
    }

    fn check_sequencing(&mut self, seqno: i64, offset: i64) -> Result<()> {
        if self.seqno == seqno && self.offset == offset {
            Ok(())
        } else if self.seqno == seqno && seqno == 0 {
            self.offset = offset;
            Ok(())
        } else {
            Err(app_error!(dt DtStatus::ERROR_INVALID, format!(
                            "BlockReadTracker: packet sequencing error: expected s={}, o={}, got s={}, o={}",
                            self.seqno, self.offset, seqno, offset
                            )))
        }
    }

    #[inline]
    fn adjust_sequencing(&mut self, dlen: usize) {
        self.seqno += 1;
        self.offset += dlen as i64;
    }

    fn validate_checksums(&self, data: &[u8], checksums: &[u8]) -> Result<()> {
        if self.checksum.is_checksum_ok(data, checksums) {
            Ok(())
        } else {
            Err(app_error!(dt DtStatus::ERROR_CHECKSUM, "BlockReadTracker: checksum error"))
        }
    }

    fn process_packet(&mut self, p: BlockDataPacket) -> Result<Bytes> {
        let (seqno, offset) = pb_decons!(PacketHeaderProto, p.header, seqno, offset_in_block);
        let _ = self.check_sequencing(seqno, offset)?;
        let _ = self.validate_checksums(&p.data, &p.checksum)?;
        self.adjust_sequencing(p.data.len());
        Ok(crop_bytes(p.data.freeze(), offset, self.read_range))
    }

    fn get_success_status(&self) -> DtStatus {
        if self.checksum.is_trivial() { DtStatus::SUCCESS } else { DtStatus::CHECKSUM_OK }
    }
}

/// Crops `Bytes` so that it fits into a target range.
/// `bs` is source `Bytes` where the source segment `[sl, sl + bs.len())` map, and `[rl, ru)`
/// is the target (bounding) range. Returns cropped `bs` (a result of intersection of the source
/// segment and the target range). The return value is empty if they do not intersect.
fn crop_bytes(mut b: Bytes, sl: i64, (rl, ru): (i64, i64)) -> Bytes {
    let su = sl + b.len() as i64;
    if su <= ru { //SR
        if rl <= sl { //rsSR
            //sS
            b
        } else if su <= rl { //Sr => sSrR
            Bytes::new()
        } else { //srSR
            //rS: cut off initial r - s bytes
            b.advance((rl - sl) as usize);
            b
        }
    } else { //RS
        if ru <= sl { //Rs => rRsS
            Bytes::new()
        } else if rl <= sl { //rs => rsRS
            //sR: cut off trailing bytes to the length R - s
            b.truncate((ru - sl) as usize);
            b
        } else { //srRS
            //rR: advance by r - s bytes and truncate to R - r bytes
            b.advance((rl - sl) as usize);
            b.truncate((ru - rl) as usize);
            b
        }
    }
}

#[test]
fn test_crop_bytes() {
    let sl = 10; //,20)
    let b = Bytes::from_static(&[10, 11, 12, 13, 14, 15, 16, 17, 18, 19]);

    macro_rules! asrt {
        ($b:expr, $rlru:expr => $($n:expr),+) => { assert_eq!(crop_bytes($b.clone(), sl, $rlru), Bytes::from_static(&[$($n),+])); };
        ($b:expr, $rlru:expr => ) => { assert_eq!(crop_bytes($b.clone(), sl, $rlru), Bytes::new()); };
    }

    asrt!(b, (0, 30) => 10, 11, 12, 13, 14, 15, 16, 17, 18, 19);
    asrt!(b, (0, 20) => 10, 11, 12, 13, 14, 15, 16, 17, 18, 19);
    asrt!(b, (10, 30) => 10, 11, 12, 13, 14, 15, 16, 17, 18, 19);
    asrt!(b, (10, 20) => 10, 11, 12, 13, 14, 15, 16, 17, 18, 19);
    asrt!(b, (20, 30) => );
    asrt!(b, (0, 10) => );
    asrt!(b, (15, 25) => 15, 16, 17, 18, 19);
    asrt!(b, (5, 15) => 10, 11, 12, 13, 14);
    asrt!(b, (11, 12) => 11);
    asrt!(b, (11, 13) => 11, 12);

    let b = Bytes::from_static(&[10]);

    asrt!(b, (0, 30) => 10);
    asrt!(b, (10, 11) => 10);
    asrt!(b, (20, 30) => );
    asrt!(b, (0, 10) => );
    asrt!(b, (15, 25) => );
    asrt!(b, (5, 15) => 10);
    asrt!(b, (11, 12) => );
    asrt!(b, (11, 13) => );

    let b = Bytes::new();
    asrt!(b, (0, 10) => );

}

use std::io::{Read, ErrorKind};

pub struct AsyncReadBlock {
    c: Connection,
    b: Bytes
}

impl AsyncReadBlock {
    pub fn new(c: Connection) -> AsyncReadBlock { AsyncReadBlock { c, b: Bytes::new() } }
    pub fn decons(self) -> (Connection, Bytes) { (self.c, self.b) }
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

#[test]
//#[ignore]
fn test_read_block_simple() {
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
    let conn_f = Connection::connect(&addr, "abc".to_owned());

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
        .and_then(|mut conn| conn.send(DtaQ::ReadBlock(rb)).map_err(|e| e.into()))
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

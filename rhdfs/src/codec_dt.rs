//--------------------------------------------------------------------------------------------------
//! Datatransfer protocol codec (standard port 50010)

use std::borrow::Cow;
//use std::result::Result as StdResult;
use std::io::Error as IoError;
use std::io::Result as IoResult;
use tokio_io::{AsyncRead, AsyncWrite};
use tokio_io::codec::{Decoder, Encoder, Framed};
use tokio_proto::streaming::pipeline::{Frame, ClientProto};
use bytes::{Bytes, BytesMut, BufMut};
use byteorder::{ByteOrder, BigEndian};
use codec_tools::*;
use ::*;
use result::ErrorConverter;

use protocolpb::proto::datatransfer::*;
use protocolpb::proto::hdfs::*;


const DATA_TRANSFER_VERSION: u16 = 28;

#[derive(Copy, Clone)]
enum Op {
    WriteBlock = 80,
    ReadBlock = 81,
    ReadMetadata = 82,
    ReplaceBlock = 83,
    CopyBlock = 84,
    BlockChecksum = 85,
    TransferBlock = 86,
    RequestShortCircuitFds = 87,
    ReleaseShortCircuitFds = 88,
    RequestShortCircuitShm = 89,
}



// A op request to a datanode:
// +-----------------------------------------------------------+
// |  Data Transfer Protocol Version, int16                    |
// +-----------------------------------------------------------+
// |  Op code, 1 byte                                          |
// +-----------------------------------------------------------+
// |  varint length + PDU                                      |
// +-----------------------------------------------------------+


fn encode_generic_op<PDU: PduSer>(op: Op, pdu: PDU, dst: &mut BytesMut) -> Result<()> {
    dst.reserve(3);
    dst.put_u16::<BigEndian>(DATA_TRANSFER_VERSION);
    dst.put_u8(op as u8);
    PduEncoder::new(VarIntU32Encoder, elen_varint_u32).encode(pdu, dst)
}

#[derive(Debug)]
pub struct BlockDataPacket {
    header: PacketHeaderProto,
    checksum: BytesMut,
    data: BytesMut
}

impl BlockDataPacket {
    pub fn is_last(&self) -> bool { self.header.get_lastPacketInBlock() || self.header.get_dataLen() == 0 }
}

#[derive(Debug)]
struct PacketLengths {
    other: u32,     //everything but the header + its length
    header: u16     //header
}

impl PacketLengths {
    const LEN: usize = U16_BYTES + U32_BYTES;
}

impl PduDes for PacketLengths {
    fn from_bytes(b: BytesMut) -> Result<Self> {
        if b.len() != PacketLengths::LEN {
            Err(app_error!{ codec "Invalid packet lengths" })
        } else {
            let other = BigEndian::read_u32(&b);
            let header = BigEndian::read_u16(&b[4..]);
            Ok(PacketLengths { other, header })
        }
    }
}

#[derive(Debug)]
struct PacketBodyDecoder {
    packet_len: usize,  //len of packet minus two leading length fields of 6 bytes in total
    header_len: usize,
    payload_len: usize  //packet_len - header_len
}

impl Decoder for PacketBodyDecoder {
    type Item = BlockDataPacket;
    type Error = Error;

    fn decode(&mut self, src: &mut BytesMut) -> Result<Option<Self::Item>> {
        if src.len() < self.packet_len {
            Ok(None)
        } else {
            let hbytes = src.split_to(self.header_len);
            let header = PacketHeaderProto::from_bytes(hbytes)?;

            let datalen = header.get_dataLen();
            if datalen as usize > self.payload_len || datalen < 0 {
                return Err(app_error!{ codec "invalid data len" })
            }
            let checksum_len = self.payload_len - datalen as usize;
            let checksum = src.split_to(checksum_len);
            let data = src.split_to(datalen as usize);
            Ok(Some(BlockDataPacket { header, checksum, data }))
        }
    }
}

#[derive(Debug)]
struct PacketDecoder {
    inner: PairDecoder<FixedSizeDecoder<PacketLengths>, PacketBodyDecoder>
}

impl PacketDecoder {
    fn new() -> PacketDecoder {
        PacketDecoder {
            inner:
            PairDecoder::new(
                FixedSizeDecoder::new_sized(PacketLengths::LEN),
                FnTailF::new(|lengths: PacketLengths| Ok(PacketBodyDecoder {
                    packet_len: lengths.other as usize - U32_BYTES + lengths.header as usize,
                    header_len: lengths.header as usize,
                    payload_len: lengths.other as usize - U32_BYTES
                })))
        }
    }
}

impl Decoder for PacketDecoder {
    type Item = BlockDataPacket;
    type Error = Error;

    fn decode(&mut self, src: &mut BytesMut) -> Result<Option<Self::Item>> {
        self.inner.decode(src)
    }
}



/// ```text
/// +-----------------------------------------------------------+
/// |  uint32 length of (this field + checksums + data)         |
/// +-----------------------------------------------------------+
/// |  uint16 length + PacketHeaderProto                        |
/// +-----------------------------------------------------------+
/// |  checksums (variable length)                              |
/// +-----------------------------------------------------------+
/// |  data                                                     |
/// +-----------------------------------------------------------+
/// ```





#[derive(Debug)]
pub enum OpBlockReadCodec {
    Head(PduDecoder<VarIntU32Decoder, BlockOpResponseProto>),
    Chunk(PacketDecoder),
    End
}

impl OpBlockReadCodec {
    fn new() -> OpBlockReadCodec {
        OpBlockReadCodec::Head(
            pdu_decoder(
                VarIntU32Decoder::new(),
                |sz| sz as usize
            )
        )
    }
}

// A op request to a datanode:
// +-----------------------------------------------------------+
// |  Data Transfer Protocol Version, int16                    |
// +-----------------------------------------------------------+
// |  Op code, 1 byte                                          |
// +-----------------------------------------------------------+
// |  varint length + OpReadBlockProto                         |
// +-----------------------------------------------------------+

impl Encoder for OpBlockReadCodec {
    type Item = Frame<OpReadBlockProto, (), IoError>;
    type Error = IoError;

    fn encode(&mut self, item: Self::Item, dst: &mut BytesMut) -> IoResult<()> {
        match item {
            Frame::Message { message, body } =>
                encode_generic_op(Op::ReadBlock, message, dst),
            _ => Err(Error::Other(Cow::from("invalid operation on OpBlockReadCodec::encode")))
        }.c_err()

    }
}


// The initial response from a datanode, in the case of reads and writes:
// +-----------------------------------------------------------+
// |  varint length + BlockOpResponseProto                     |
// +-----------------------------------------------------------+
//
//  org.apache.hadoop.hdfs.protocol.datatransfer.Sender.send(
//    final DataOutputStream out, final Op opcode, final Message proto)
//  org.apache.hadoop.hdfs.server.datanode.DataXceiver.readBlock(...)

impl Decoder for OpBlockReadCodec {
    type Item = Frame<BlockOpResponseProto, BlockDataPacket, IoError>;
    type Error = IoError;

    /// The initial response from a datanode, in the case of reads and writes:
    /// ```text
    /// +-----------------------------------------------------------+
    /// |  varint length + BlockOpResponseProto                     |
    /// +-----------------------------------------------------------+
    /// ```
    /// Chunks:
    ///
    /// ```text
    /// +-----------------------------------------------------------+
    /// |  uint32 length of (this field + checksums + data)         |
    /// +-----------------------------------------------------------+
    /// |  uint16 length + PacketHeaderProto                        |
    /// +-----------------------------------------------------------+
    /// |  checksums (variable length)                              |
    /// +-----------------------------------------------------------+
    /// |  data                                                     |
    /// +-----------------------------------------------------------+
    /// ```
    /// See `org.apache.hadoop.hdfs.server.datanode.BlockSender.sendPacket`
    /// See `org.apache.hadoop.hdfs.protocol.datatransfer.PacketReceiver.doRead`
    /// See `org.apache.hadoop.hdfs.protocol.datatransfer.PacketHeader`
    fn decode(&mut self, src: &mut BytesMut) -> IoResult<Option<Self::Item>> {
        use codec_dt::OpBlockReadCodec as O;
        use util::*;

        let rv = logging_switch_state_f(self,
            |s| match s {
                &mut O::Head(ref mut rdr) =>
                    match rdr.decode(src) {
                        Ok(Some(w)) => SnV::SV(
                            O::Chunk(PacketDecoder::new()),
                            Ok(Some(Frame::Message { message: w, body: true }))
                        ),
                        Ok(None) => SnV::V(Ok(None)),
                        Err(e) => SnV::V(Err(e))
                    },
                &mut O::Chunk(ref mut rdr) =>
                    match rdr.decode(src) {
                        Ok(Some(w)) => SnV::SV(
                            if w.is_last() { O::End } else { O::Chunk(PacketDecoder::new()) },
                            Ok(Some(Frame::Body { chunk: Some(w) })),
                        ),
                        Ok(None) => SnV::V(Ok(None)),
                        Err(e) => SnV::V(Err(e))
                    },
                &mut O::End =>
                    SnV::V(Ok(Some(Frame::Body { chunk: None })))
            }
        );
        rv.c_err()
    }

}


struct OpBlockReadTProto;


impl<T: AsyncRead + AsyncWrite + 'static> ClientProto<T> for OpBlockReadTProto {
    type Request = OpReadBlockProto;
    type RequestBody = ();
    type Response = BlockOpResponseProto;
    type ResponseBody = BlockDataPacket;
    type Error = IoError;

    type Transport = Framed<T, OpBlockReadCodec>;
    type BindTransport = IoResult<Self::Transport>;

    fn bind_transport(&self, io: T) -> Self::BindTransport {
        let codec = OpBlockReadCodec::new();
        Ok(io.framed(codec))
    }
}


use tokio_proto::util::client_proxy::ClientProxy;
use tokio_proto::TcpClient;
use tokio_proto::streaming::{Body, Message};
use tokio_service::Service;
use futures::Future;
use std::net::SocketAddr;
use tokio_core::reactor::{Core, Handle};

/// Message type used to communicate with tokio-proto. The library should hide
/// this and instead expose a custom message type
type UpstreamMessage = Message<OpReadBlockProto, Body<(), IoError>>;
type DownstreamMessage = Message<BlockOpResponseProto, Body<BlockDataPacket, IoError>>;

/// external message types
pub struct ReadBlock {
    block_id: u64
}

#[derive(Debug)]
pub struct ReadBlockResponse {
    ok: bool,
    m: String,
    s: Option<DataPacketStream>
}

impl From<ReadBlock> for UpstreamMessage {
    fn from(rb: ReadBlock) -> Self {
        let mut orbp = OpReadBlockProto::new();
        //orbp.set_len(9964);
        orbp.set_len(5);
        orbp.set_offset(0);
        let mut h = ClientOperationHeaderProto::new();
        let mut bh = BaseHeaderProto::new();
        let mut ebp = ExtendedBlockProto::new();


        ebp.set_blockId(1073742835);
        ebp.set_generationStamp(2015);
        ebp.set_numBytes(5);
        ebp.set_poolId("BP-1914853243-127.0.0.1-1500467607052".to_owned());
        /*
        ebp.set_blockId(1073742834);
        ebp.set_generationStamp(2014);
        ebp.set_numBytes(9964);
        ebp.set_poolId("BP-1914853243-127.0.0.1-1500467607052".to_owned());

        */
        bh.set_block(ebp);
        //bh.set_token()
        h.set_baseHeader(bh);
        h.set_clientName("abc".to_owned());
        orbp.set_header(h);
        Message::WithoutBody(orbp)
    }
}

#[derive(Debug)]
pub struct DataPacketStream {
    inner: Body<BlockDataPacket, IoError>
}

use futures::{Stream, Poll};


impl Stream for DataPacketStream {
    type Item = BlockDataPacket;
    type Error = IoError;

    fn poll(&mut self) -> Poll<Option<BlockDataPacket>, IoError> {
        self.inner.poll()
    }
}

impl From<DownstreamMessage> for ReadBlockResponse {
    fn from(m: DownstreamMessage) -> Self {
        match m {
            Message::WithoutBody(m) => ReadBlockResponse { ok: false, m: format!("{:?}", m), s: None },
            Message::WithBody(m, b) => ReadBlockResponse { ok: true, m: format!("{:?}", m), s: Some(DataPacketStream { inner: b }) }
        }
    }
}

pub struct Client {
    inner: ClientTypeMap<ClientProxy<UpstreamMessage, DownstreamMessage, IoError>>,
}

struct ClientTypeMap<T> {
    inner: T,
}

impl<T> Service for ClientTypeMap<T>
    where T: Service<Request = UpstreamMessage, Response = DownstreamMessage, Error = IoError>,
          T::Future: 'static
{
    type Request = ReadBlock;
    type Response = ReadBlockResponse;
    type Error = IoError;
    type Future = Box<Future<Item = ReadBlockResponse, Error = IoError>>;

    fn call(&self, req: ReadBlock) -> Self::Future {
        Box::new(self.inner.call(req.into())
            .map(ReadBlockResponse::from))
    }
}

impl Client {
    pub fn connect(addr: &SocketAddr, handle: &Handle) -> Box<Future<Item = Client, Error = IoError>> {
        let ret = TcpClient::new(OpBlockReadTProto)
            .connect(addr, handle)
            .map(|client_proxy| {
                // Wrap the returned client handle with our `ClientTypeMap`
                // service middleware
                let type_map = ClientTypeMap { inner: client_proxy };
                Client { inner: type_map }
            });
        Box::new(ret)
    }
}



impl Service for Client {
    type Request = ReadBlock;
    type Response = ReadBlockResponse;
    type Error = IoError;
    // For simplicity, box the future.
    type Future = Box<Future<Item = ReadBlockResponse, Error = IoError>>;

    fn call(&self, req: ReadBlock) -> Self::Future {
        self.inner.call(req)
    }
}

pub fn do_things() -> Result<()> {
    use std::net::ToSocketAddrs;
    use tokio_core::reactor::Core;
    use std::borrow::Cow;
    use futures::Future;

    let mut core = Core::new().unwrap();
    let addr = "127.0.0.1:50010".to_socket_addrs().unwrap().next().ok_or(Error::Other(Cow::from("NN host not found"))).unwrap();
    let handle = core.handle();
    let conn = Client::connect(&addr, &core.handle());

    let req = conn.and_then(|cl| cl.call(ReadBlock { block_id: 0 }));

    let res = core.run(req).unwrap();
    println!("RESP: {:?}", res);

    if let Some(st) = res.s {
        let mut s = st;

        while let (Some(p), ns) = core.run(s.into_future()).unwrap() {
            println!("PACKET, cksum_len={}, data_len={}", p.checksum.len(), p.data.len());
            s = ns;
        }
    }
    //TODO: Respond with ClientReadStatusProto
    //see DataXceiver::readBlock

    Ok(())
}

#[cfg(test)]
trait ToBytes {
    fn to_bytes(&self) -> Vec<u8>;
}

#[cfg(test)]
impl ToBytes for str {
    fn to_bytes(&self) -> Vec<u8> {
        enum S {
            N,
            B(u8)
        };

        let mut rv = Vec::new();

        self.chars().fold(S::N, |s, ch| match (s, ch) {
            (S::N, c) if c.is_digit(16) => S::B(c.to_digit(16).unwrap() as u8),
            (S::B(b), c) if c.is_digit(16) => {
                rv.push((b << 4) | c.to_digit(16).unwrap() as u8);
                S::N
            },
            (S::N, ':') => S::N,
            (S::N, c) if c == ' ' || c == '\x0a' => S::N,
            _ => panic!("Invalid hex string")
        });
        rv
    }
}


use std::fmt;

struct HexSlice<'a>(&'a [u8]);

impl<'a> HexSlice<'a> {
    fn new<T>(data: &'a T) -> HexSlice<'a>
        where T: ?Sized + AsRef<[u8]> + 'a
    {
        HexSlice(data.as_ref())
    }
}

impl<'a> fmt::Display for HexSlice<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for byte in self.0 {
            write!(f, "{:02x} ", byte)?;
        }
        Ok(())
    }
}

#[test]
fn test_from_bytes() {
    assert_eq!("00:01:02".to_bytes(), vec![0x00u8, 0x01, 0x02]);
    assert_eq!("\
    00:
    01:
    02:
    03".to_bytes(), vec![0x00u8, 0x01, 0x02, 0x03]);
}

#[test]
fn test_OpBlockReadCodec_decoder() {
    let mut b = BytesMut::new();
    b.extend_from_slice(&//initial response
"
0d:
08:00:22:09:
0a:05:08:02:
10:80:04:10:
00
".to_bytes());
    b.extend_from_slice(&//packet1 - h+cks
        "
00:00:00:0d:
00:19:
09:00:00:00:
00:00:00:00:
00:11:00:00:
00:00:00:00:
00:00:18:00:
25:05:00:00:
00:
a9:c7:c0:1b
".to_bytes());
    b.extend_from_slice(&//packet1 - data
"
41:42:43:44:0a
".to_bytes());
    b.extend_from_slice(&//packet2 - empty
"
00:00:00:04:
00:19:
09:05:00:00:
00:00:00:00:
00:11:01:00:
00:00:00:00:
00:00:18:01:
25:00:00:00:
00
".to_bytes());

    let mut c = OpBlockReadCodec::new();
    println!("1C. {:?}", c);
    println!("1B. {}", HexSlice(&b));
    println!("1R. {:?}", c.decode(&mut b));
    println!("2C. {:?}", c);
    println!("2B. {}", HexSlice(&b[..]));
    println!("2R. {:?}", c.decode(&mut b));
    println!("3C. {:?}", c);
    println!("3B. {}", HexSlice(&b[..]));
    println!("3R. {:?}", c.decode(&mut b));
    println!("4C. {:?}", c);
    println!("4B. {}", HexSlice(&b[..]));
    println!("4R. {:?}", c.decode(&mut b));
    println!("5C. {:?}", c);
    println!("5B. {}", HexSlice(&b[..]));

}









//-------------------------------------------------------------------------------------------------
// Old impl


// A op request to a datanode:
// +-----------------------------------------------------------+
// |  Data Transfer Protocol Version, int16                    |
// +-----------------------------------------------------------+
// |  Op code, 1 byte                                          |
// +-----------------------------------------------------------+
// |  varint length + OpReadBlockProto                         |
// +-----------------------------------------------------------+
//
// org.apache.hadoop.hdfs.server.datanode.DataXceiver.run()

//fn request_packet(payload: Vec<u8>) -> Result<Vec<u8>> {
//}

// The initial response from a datanode, in the case of reads and writes:
// +-----------------------------------------------------------+
// |  varint length + BlockOpResponseProto                     |
// +-----------------------------------------------------------+
//
//  org.apache.hadoop.hdfs.protocol.datatransfer.Sender.send(
//    final DataOutputStream out, final Op opcode, final Message proto)
//  org.apache.hadoop.hdfs.server.datanode.DataXceiver.readBlock(...)

//! Datatransfer protocol codec (standard port 50010)


use std::borrow::Cow;
use tokio_io::codec::{Decoder, Encoder};
use bytes::{BytesMut, BufMut};
use byteorder::BigEndian;


use *;
use super::packet::{BlockDataPacket, PacketDecoder};
use protobuf_api::*;
use codec_tools::*;

const DATA_TRANSFER_VERSION: u16 = 28;

#[allow(dead_code)]
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

/// An operation request to a datanode:
/// ```text
/// +-----------------------------------------------------------+
/// |  Data Transfer Protocol Version, int16                    |
/// +-----------------------------------------------------------+
/// |  Op code, 1 byte                                          |
/// +-----------------------------------------------------------+
/// |  varint length + PDU                                      |
/// +-----------------------------------------------------------+
/// ```
fn encode_generic_op<PDU: PduSer>(op: Op, pdu: PDU, dst: &mut BytesMut) -> Result<()> {
    dst.reserve(3);
    dst.put_u16::<BigEndian>(DATA_TRANSFER_VERSION);
    dst.put_u8(op as u8);
    encoder::varint_u32().encode(pdu, dst)
}



#[derive(Debug)]
pub enum OpBlockReadCodec {
    Head(PduDecoder<VarIntU32Decoder, BlockOpResponseProto>),
    Chunk(PacketDecoder),
    End
}

#[derive(Debug, PartialEq)]
pub enum OpBlockReadMessage {
    Initial(bool, BlockOpResponseProto),
    Packet(BlockDataPacket),
    End
}

fn has_data(borp: &BlockOpResponseProto) -> bool {
    pb_decons!(BlockOpResponseProto, borp, status) == DtStatus::SUCCESS
}


impl OpBlockReadCodec {
    pub fn new() -> OpBlockReadCodec {
        OpBlockReadCodec::Head(decoder::varint_u32())
    }

    /// The initial response from a datanode, in the case of reads and writes:
    /// ```text
    /// +-----------------------------------------------------------+
    /// |  varint length + BlockOpResponseProto                     |
    /// +-----------------------------------------------------------+
    /// ```
    /// Chunks are Data transfer packets

    fn decode(&mut self, src: &mut BytesMut) -> IoResult<Option<OpBlockReadMessage>> {
        use self::OpBlockReadCodec as O;
        use util::*;

        logging_switch_state_f("OpBlockReadCodec", self,
                               |s| match s {
                                   &mut O::Head(ref mut rdr) =>
                                       match rdr.decode(src) {
                                           Ok(Some(w)) => SnV::SV(
                                               O::Chunk(PacketDecoder::new()),
                                               Ok(Some(OpBlockReadMessage::Initial(has_data(&w), w)))
                                           ),
                                           Ok(None) => SnV::V(Ok(None)),
                                           Err(e) => SnV::V(Err(e))
                                       },
                                   &mut O::Chunk(ref mut rdr) =>
                                       match rdr.decode(src) {
                                           Ok(Some(w)) => SnV::SV(
                                               if w.is_last() { O::End } else { O::Chunk(PacketDecoder::new()) },
                                               Ok(Some(OpBlockReadMessage::Packet(w))),
                                           ),
                                           Ok(None) => SnV::V(Ok(None)),
                                           Err(e) => SnV::V(Err(e))
                                       },
                                   &mut O::End =>
                                       SnV::V(Ok(Some(OpBlockReadMessage::End)))
                               }
        ).c_err()
    }

}

#[derive(Debug)]
pub enum DtReq {
    ReadBlock(OpReadBlockProto),
    ClientReadStatus(ClientReadStatusProto)
}

impl DtReq {
    pub fn set_client_name(&mut self, client_name: &str) {
        match self {
            &mut DtReq::ReadBlock(ref mut orbp) => orbp.mut_header().set_clientName(client_name.to_owned()),
            &mut DtReq::ClientReadStatus(..) => ()
        }
    }
}

#[derive(Debug)]
pub enum DtRsp {
    ReadBlock(OpBlockReadMessage)
}

impl DtRsp {
    fn terminates_exchange(&self) -> bool {
        match self {
            &DtRsp::ReadBlock(OpBlockReadMessage::Initial(has_data, _)) => !has_data,
            &DtRsp::ReadBlock(OpBlockReadMessage::Packet(_)) => false,
            //We assume majority are one req-one resp
            _ => true

        }
    }
}

pub enum DtCodec {
    Null,
    OpBlockRead(OpBlockReadCodec)
}

impl DtCodec {
    pub fn new() -> DtCodec { DtCodec::Null }
}

impl Encoder for DtCodec {
    type Item = DtReq;
    type Error = IoError;

    fn encode(&mut self, item: Self::Item, dst: &mut BytesMut) -> IoResult<()> {
        let (s, e) = match item {
            DtReq::ReadBlock(orbp) => (
                DtCodec::OpBlockRead(OpBlockReadCodec::new()),
                encode_generic_op(Op::ReadBlock, orbp, dst)
            ),

            DtReq::ClientReadStatus(crs) => (
                DtCodec::Null,
                encoder::varint_u32().encode(crs, dst)
            ),

            //_ => (
            //    DtCodec::Null,
            //    Err(app_error!(other "invalid operation on OpBlockReadCodec::encode"))
            //),

        };
        *self = s;
        e.c_err()
    }
}

impl Decoder for DtCodec {
    type Item = DtRsp;
    type Error = IoError;

    /// The initial response from a datanode, in the case of reads and writes:
    /// ```text
    /// +-----------------------------------------------------------+
    /// |  varint length + BlockOpResponseProto                     |
    /// +-----------------------------------------------------------+
    /// ```
    /// Chunks are Data transfer packets

    fn decode(&mut self, src: &mut BytesMut) -> IoResult<Option<Self::Item>> {
        let rv = match self {
            &mut DtCodec::OpBlockRead(ref mut obrc) =>
                obrc.decode(src).map(|w| w.map(|v|
                    DtRsp::ReadBlock(v)
                )),
            &mut DtCodec::Null =>
                Err(app_error!(codec "DtCodec: invalid protocol state").into())
        };
        //switch state to Null once finished with decoding
        if let &Ok(Some(ref rsp)) = &rv {
            if rsp.terminates_exchange() {
                *self = DtCodec::Null;
            }
        }
        rv
    }
}



#[test]
fn test_block_read_codec() {
    use util::test::*;
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
    //1C. Head(H(VarIntU32Decoder { cur: 0, bit: 0, pos: 0 }, FixedSizeDecoderTailF { f: 0x7ff7e37d1560 }))
    //println!("1C. {:?}", c);

    //1B. 0d 08 00 22 09 0a 05 08 02 10 80 04 10 00 00 00 00 0d 00 19 09 00 00 00 00 00 00 00 00 11 00 00 00 00 00 00 00 00 18 00 25 05 00 00 00 a9 c7 c0 1b 41 42 43 44 0a 00 00 00 04 00 19 09 05 00 00 00 00 00 00 00 11 01 00 00 00 00 00 00 00 18 01 25 00 00 00 00
    //println!("1B. {}", HexSlice::new(&b));
    assert_eq!("\
    0d 08 00 22 09 0a 05 08 02 10 80 04 10 00 00 00 00 0d 00 19 09 00 00 00 00 00 00 00 00 11 \
    00 00 00 00 00 00 00 00 18 00 25 05 00 00 00 a9 c7 c0 1b 41 42 43 44 0a 00 00 00 04 00 19 \
    09 05 00 00 00 00 00 00 00 11 01 00 00 00 00 00 00 00 18 01 25 00 00 00 00"
                   .to_bytes(),
               &b[..]
    );

    let r = c.decode(&mut b);
    //1R. Ok(Some(InitialResponse(status: SUCCESS readOpChecksumInfo {checksum {type: CHECKSUM_CRC32C bytesPerChecksum: 512} chunkOffset: 0})))
    //println!("1R. {:?}", r);
    assert_eq!(
        r.ok(),
        Some(Some(OpBlockReadMessage::Initial(true, pb_cons!(BlockOpResponseProto,
            status: DtStatus::SUCCESS,
            read_op_checksum_info: pb_cons!(ReadOpChecksumInfoProto,
                checksum: pb_cons!(ChecksumProto,
                    type: ChecksumTypeProto::CHECKSUM_CRC32C,
                    bytes_per_checksum: 512
                ),
                chunk_offset: 0
            )
        ))))
    );

    //2C. Chunk(PacketDecoder { inner: H(FixedSizeDecoder { sz: 6, pdu_type: PhantomData }, FnTailF { f: 0x7ff7e37d10b0 }) })
    //println!("2C. {:?}", c);

    //2B. 00 00 00 0d 00 19 09 00 00 00 00 00 00 00 00 11 00 00 00 00 00 00 00 00 18 00 25 05 00 00 00 a9 c7 c0 1b 41 42 43 44 0a 00 00 00 04 00 19 09 05 00 00 00 00 00 00 00 11 01 00 00 00 00 00 00 00 18 01 25 00 00 00 00
    //println!("2B. {}", HexSlice::new(&b[..]));
    assert_eq!("\
    00 00 00 0d 00 19 09 00 00 00 00 00 00 00 00 11 00 00 00 00 00 00 00 00 18 00 25 05 00 00 00 \
    a9 c7 c0 1b 41 42 43 44 0a 00 00 00 04 00 19 09 05 00 00 00 00 00 00 00 11 01 00 00 00 00 00 \
    00 00 18 01 25 00 00 00 00"
                   .to_bytes(),
               &b[..]
    );

    let r = c.decode(&mut b);
    //2R. Ok(Some(Packet(BlockDataPacket { header: offsetInBlock: 0 seqno: 0 lastPacketInBlock: false dataLen: 5, checksum: b"\xa9\xc7\xc0\x1b", data: b"ABCD\n" })))
    //println!("2R. {:?}", r);
    assert_eq!(
        r.ok(),
        Some(Some(
            OpBlockReadMessage::Packet(BlockDataPacket {
                header: pb_cons!(PacketHeaderProto,
                    offset_in_block: 0,
                    seqno: 0,
                    last_packet_in_block: false,
                    data_len: 5
                ),
                checksum: BytesMut::from(vec![0xa9, 0xc7, 0xc0, 0x1b]),
                data: BytesMut::from("ABCD\n")
            })
        ))

    );

    //3C. Chunk(PacketDecoder { inner: H(FixedSizeDecoder { sz: 6, pdu_type: PhantomData }, FnTailF { f: 0x7ff7e37d10b0 }) })
    //println!("3C. {:?}", c);

    //3B. 00 00 00 04 00 19 09 05 00 00 00 00 00 00 00 11 01 00 00 00 00 00 00 00 18 01 25 00 00 00 00
    //println!("3B. {}", HexSlice::new(&b[..]));
    assert_eq!(
        "00 00 00 04 00 19 09 05 00 00 00 00 00 00 00 11 01 00 00 00 00 00 00 00 18 01 25 00 00 00 00"
            .to_bytes(),
        &b[..]
    );
    let _ = c.decode(&mut b);
    //3R. Ok(Some(Packet(BlockDataPacket { header: offsetInBlock: 5 seqno: 1 lastPacketInBlock: true dataLen: 0, checksum: b"", data: b"" })))
    //println!("3R. {:?}", r);

    //4C. End
    //println!("4C. {:?}", c);

    //4B.
    //println!("4B. {}", HexSlice::new(&b[..]));
    assert!(b.is_empty());

    let _ = c.decode(&mut b);
    //4R. Ok(Some(End))
    //println!("4R. {:?}", r);

    //5C. End
    //println!("5C. {:?}", c);

    //5B.
    //println!("5B. {}", HexSlice::new(&b[..]));
    assert!(b.is_empty());
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
//
// The initial response from a datanode, in the case of reads and writes:
// +-----------------------------------------------------------+
// |  varint length + BlockOpResponseProto                     |
// +-----------------------------------------------------------+
//
//  org.apache.hadoop.hdfs.protocol.datatransfer.Sender.send(
//    final DataOutputStream out, final Op opcode, final Message proto)
//  org.apache.hadoop.hdfs.server.datanode.DataXceiver.readBlock(...)

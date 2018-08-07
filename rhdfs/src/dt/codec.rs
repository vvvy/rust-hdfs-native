//! Datatransfer protocol codec (standard port 50010)


use std::borrow::Cow;
use tokio_io::codec::{Decoder, Encoder};
use bytes::{BytesMut, BufMut};
use byteorder::BigEndian;


use *;
use super::packet::{BlockDataPacket, PacketDecoder, PacketEncoder};
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
    Chunk(PacketDecoder)
}

#[derive(Debug, PartialEq)]
pub enum OpBlockReadMessage {
    /// `Initial(has_data, m)`
    Initial(bool, BlockOpResponseProto),
    /// `Packet(m, is_last)`
    Packet(BlockDataPacket, bool)
}

fn has_data(borp: &BlockOpResponseProto) -> bool {
    pb_decons!(BlockOpResponseProto, borp, status) == DtStatus::SUCCESS
}

/// A generalization of `IoResult<Option<R>>`, adding `ReadyLast` variant signaling to end streaming
/// mode and return to `Null` topleve mode
#[derive(Debug)]
enum DecoderResult<M> {
    /// Message decoding complete, and this is not the last message in the streaming sequence
    ReadyNotLast(M),
    /// Message decoding complete, and this is the last message in the streaming sequence
    ReadyLast(M),
    /// Message decoding incomplete
    NotReady,
    /// Message decoding error
    Err(Error),
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

    fn decode(&mut self, src: &mut BytesMut) -> DecoderResult<OpBlockReadMessage> {
        use self::OpBlockReadCodec as O;
        use self::DecoderResult as R;
        use util::*;

        switch_state_t("OpBlockReadCodec", self,
                       |s| match s {
                           &mut O::Head(ref mut rdr) =>
                               match rdr.decode(src) {
                                   Ok(Some(w)) => if has_data(&w) {
                                       SnV::SV(
                                           O::Chunk(PacketDecoder::new()),
                                           R::ReadyNotLast(OpBlockReadMessage::Initial(true, w)),
                                       )
                                   } else {
                                       SnV::V(R::ReadyLast(OpBlockReadMessage::Initial(false, w)))
                                   },
                                   Ok(None) => SnV::V(R::NotReady),
                                   Err(e) => SnV::V(R::Err(e))
                               },
                           &mut O::Chunk(ref mut rdr) =>
                               match rdr.decode(src) {
                                   Ok(Some(w)) => if w.is_last() {
                                       SnV::V(R::ReadyLast(OpBlockReadMessage::Packet(w, true)))
                                   } else {
                                       SnV::SV(
                                           O::Chunk(PacketDecoder::new()),
                                           R::ReadyNotLast(OpBlockReadMessage::Packet(w, false)),
                                       )
                                   },
                                   Ok(None) => SnV::V(R::NotReady),
                                   Err(e) => SnV::V(R::Err(e))
                               }
                       },
        )
    }
}


#[derive(Debug)]
pub enum OpBlockWriteCodec {
    Head(PduDecoder<VarIntU32Decoder, BlockOpResponseProto>),
    Ack(PduDecoder<VarIntU32Decoder, PipelineAckProto>, i64)
}

#[derive(Debug, PartialEq)]
pub enum OpBlockWriteMessage {
    Initial(bool, BlockOpResponseProto),
    Ack(PipelineAckProto)
}


const UNKOWN_SEQNO: i64 = -1;

impl OpBlockWriteCodec {
    pub fn new() -> OpBlockWriteCodec {
        OpBlockWriteCodec::Head(decoder::varint_u32())
    }

    /// The initial response from a datanode, in the case of reads and writes:
    /// ```text
    /// +-----------------------------------------------------------+
    /// |  varint length + BlockOpResponseProto                     |
    /// +-----------------------------------------------------------+
    /// ```
    /// Chunks are Data transfer packets

    fn decode(&mut self, src: &mut BytesMut) -> DecoderResult<OpBlockWriteMessage> {
        use self::OpBlockWriteCodec as O;
        use self::DecoderResult as R;
        use util::*;

        switch_state_t("OpBlockWriteCodec", self,
                       |s| match s {
                           &mut O::Head(ref mut rdr) =>
                               match rdr.decode(src) {
                                   Ok(Some(w)) => if has_data(&w) {
                                       SnV::SV(
                                           O::Ack(decoder::varint_u32(), UNKOWN_SEQNO),
                                           R::ReadyNotLast(OpBlockWriteMessage::Initial(true, w)),
                                       )
                                   } else {
                                       SnV::V(R::ReadyLast(OpBlockWriteMessage::Initial(false, w)))
                                   },
                                   Ok(None) => SnV::V(R::NotReady),
                                   Err(e) => SnV::V(R::Err(e))
                               },
                           &mut O::Ack(ref mut rdr, last_seqno) =>
                               match rdr.decode(src) {
                                   Ok(Some(w)) => if pb_get!(PipelineAckProto, w, seqno) == last_seqno {
                                       SnV::V(R::ReadyLast(OpBlockWriteMessage::Ack(w)))
                                   } else {
                                       SnV::SV(
                                           O::Ack(decoder::varint_u32(), last_seqno),
                                           R::ReadyNotLast(OpBlockWriteMessage::Ack(w)),
                                       )
                                   },
                                   Ok(None) => SnV::V(R::NotReady),
                                   Err(e) => SnV::V(R::Err(e))
                               }
                       },
        )
    }
}



#[derive(Debug)]
pub enum DtQ {
    ReadBlock(OpReadBlockProto),
    ClientReadStatus(ClientReadStatusProto),
    WriteBlock(OpWriteBlockProto),
    Packet(BlockDataPacket)
}

impl DtQ {
    pub fn set_client_name(&mut self, client_name: &str) {
        match self {
            &mut DtQ::ReadBlock(ref mut orbp) => orbp.mut_header().set_clientName(client_name.to_owned()),
            &mut DtQ::WriteBlock(ref mut owbp) => owbp.mut_header().set_clientName(client_name.to_owned()),
            &mut DtQ::Packet(..) => (),
            &mut DtQ::ClientReadStatus(..) => ()
        }
    }
}

#[derive(Debug)]
pub enum DtR {
    ReadBlock(OpBlockReadMessage),
    WriteBlock(OpBlockWriteMessage)
}

#[derive(Debug)]
pub enum DtCodec {
    Null,
    OpBlockRead(OpBlockReadCodec),
    OpBlockWrite(OpBlockWriteCodec)
}

impl DtCodec {
    pub fn new() -> DtCodec { DtCodec::Null }
}

impl Encoder for DtCodec {
    type Item = DtQ;
    type Error = Error;

    fn encode(&mut self, item: Self::Item, dst: &mut BytesMut) -> Result<()> {
        let (s, e) = match item {
            DtQ::ReadBlock(orbp) => (
                Some(DtCodec::OpBlockRead(OpBlockReadCodec::new())),
                encode_generic_op(Op::ReadBlock, orbp, dst)
            ),

            DtQ::ClientReadStatus(crs) => (
                Some(DtCodec::Null),
                encoder::varint_u32().encode(crs, dst)
            ),

            DtQ::WriteBlock(owbp) => (
                Some(DtCodec::OpBlockWrite(OpBlockWriteCodec::new())),
                encode_generic_op(Op::WriteBlock, owbp, dst)
            ),

            DtQ::Packet(bdp) => match self {
                &mut DtCodec::OpBlockWrite(OpBlockWriteCodec::Ack(_, ref mut seqno)) => {
                    //update sequence number
                    *seqno = bdp.seq_no();
                    (None, PacketEncoder::new().encode(bdp, dst))
                }
                _ => return Err(app_error!(codec "Invalid codec state: {:?}/Packet({:?})", self, bdp).into())
            }

            //_ => (
            //    DtCodec::Null,
            //    Err(app_error!(other "invalid operation on OpBlockReadCodec::encode"))
            //),

        };
        if let Some(s) = s { *self = s };
        //e.c_err()
        e
    }
}

impl Decoder for DtCodec {
    type Item = DtR;
    type Error = Error;

    /// The initial response from a datanode, in the case of reads and writes:
    /// ```text
    /// +-----------------------------------------------------------+
    /// |  varint length + BlockOpResponseProto                     |
    /// +-----------------------------------------------------------+
    /// ```
    /// Chunks are Data transfer packets

    fn decode(&mut self, src: &mut BytesMut) -> Result<Option<Self::Item>> {
        use self::DecoderResult as R;

        if !src.is_empty() {
            let (rv, goto_null) = match self {
                &mut DtCodec::OpBlockRead(ref mut obrc) =>
                    match obrc.decode(src) {
                        R::ReadyNotLast(v) =>
                            (Ok(Some(DtR::ReadBlock(v))), false),
                        R::ReadyLast(v) =>
                            (Ok(Some(DtR::ReadBlock(v))), true),
                        R::NotReady =>
                            (Ok(None), false),
                        R::Err(e) =>
                            (Err(e), true)
                    },
                &mut DtCodec::OpBlockWrite(ref mut obwc) =>
                    match obwc.decode(src) {
                        R::ReadyNotLast(v) =>
                            (Ok(Some(DtR::WriteBlock(v))), false),
                        R::ReadyLast(v) =>
                            (Ok(Some(DtR::WriteBlock(v))), true),
                        R::NotReady =>
                            (Ok(None), false),
                        R::Err(e) =>
                            (Err(e), true)
                    },
                &mut DtCodec::Null =>
                    (Err(app_error!(codec "DtCodec: invalid protocol state")), false)
            };
            //switch state to Null once finished with decoding
            if goto_null {
                *self = DtCodec::Null;
            }
            rv
        } else {
            Ok(None)
        }
    }
}



#[test]
fn test_block_read_codec() {
    use util::test::*;
    use self::DecoderResult as R;

    fn not_last<W>(r: R<W>) -> Option<W> {
        match r {
            R::ReadyNotLast(w) => Some(w),
            _ => None
        }
    }

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
        not_last(r),
        Some(OpBlockReadMessage::Initial(true, pb_cons!(BlockOpResponseProto,
            status: DtStatus::SUCCESS,
            read_op_checksum_info: pb_cons!(ReadOpChecksumInfoProto,
                checksum: pb_cons!(ChecksumProto,
                    type: ChecksumTypeProto::CHECKSUM_CRC32C,
                    bytes_per_checksum: 512
                ),
                chunk_offset: 0
            )
        )))
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
        not_last(r),
        Some(
            OpBlockReadMessage::Packet(BlockDataPacket {
                header: pb_cons!(PacketHeaderProto,
                    offset_in_block: 0,
                    seqno: 0,
                    last_packet_in_block: false,
                    data_len: 5
                ),
                checksum: BytesMut::from(vec![0xa9, 0xc7, 0xc0, 0x1b]),
                data: BytesMut::from("ABCD\n")
            }, false)
        )
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

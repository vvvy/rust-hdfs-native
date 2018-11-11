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
pub enum OBR {
    Head(PduDecoder<VarIntU32Decoder, BlockOpResponseProto>),
    Chunk(PacketDecoder)
}

impl OBR {
    pub fn new() -> OBR {
        OBR::Head(decoder::varint_u32())
    }
}

#[derive(Debug)]
pub enum OBW {
    Head(PduDecoder<VarIntU32Decoder, BlockOpResponseProto>),
    Ack(PduDecoder<VarIntU32Decoder, PipelineAckProto>)
}

impl OBW {
    pub fn new() -> OBW {
        OBW::Head(decoder::varint_u32())
    }
}

#[derive(Debug)]
pub enum DtQ {
    ReadBlock(OpReadBlockProto),
    ClientReadStatus(ClientReadStatusProto),
    WriteBlock(OpWriteBlockProto),
    Packet(BlockDataPacket)
}

#[derive(Debug, PartialEq)]
pub enum DtR {
    ReadBlockResponse(BlockOpResponseProto),
    Packet(BlockDataPacket),
    WriteBlockResponse(BlockOpResponseProto),
    Ack(PipelineAckProto)
}

#[derive(Debug)]
pub enum DtCodec {
    Null,
    OpBlockRead(OBR),
    OpBlockWrite(OBW)
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
                Some(DtCodec::OpBlockRead(OBR::new())),
                encode_generic_op(Op::ReadBlock, orbp, dst)
            ),
            DtQ::ClientReadStatus(crs) => (
                Some(DtCodec::Null),
                encoder::varint_u32().encode(crs, dst)
            ),
            DtQ::WriteBlock(owbp) => (
                Some(DtCodec::OpBlockWrite(OBW::new())),
                encode_generic_op(Op::WriteBlock, owbp, dst)
            ),
            DtQ::Packet(bdp) => (
                None,
                PacketEncoder::new().encode(bdp, dst)
            )
        };
        if let Some(s) = s { *self = s };
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
        //use self::DecoderResult as R;

        let rv = if !src.is_empty() {
            match self {
                DtCodec::OpBlockRead(OBR::Head(rdr)) =>
                    rdr.decode(src).map(|o|
                        o.map(|v| DtR::ReadBlockResponse(v))
                    ),
                DtCodec::OpBlockRead(OBR::Chunk(rdr)) =>
                    rdr.decode(src).map(|o|
                        o.map(|v| DtR::Packet(v))
                    ),
                DtCodec::OpBlockWrite(OBW::Head(rdr)) =>
                    rdr.decode(src).map(|o|
                        o.map(|v| DtR::WriteBlockResponse(v))
                    ),
                DtCodec::OpBlockWrite(OBW::Ack(rdr)) =>
                    rdr.decode(src).map(|o|
                        o.map(|v| DtR::Ack(v))
                    ),
                DtCodec::Null =>
                    Err(app_error!(codec "DtCodec: invalid protocol state"))
            }
        } else {
            Ok(None)
        };

        match &rv {
            Ok(Some(DtR::ReadBlockResponse(..))) | Ok(Some(DtR::Packet(..)))  =>
                *self = DtCodec::OpBlockRead(OBR::Chunk(PacketDecoder::new())),
            Ok(Some(DtR::WriteBlockResponse(..))) | Ok(Some(DtR::Ack(..))) =>
                *self = DtCodec::OpBlockWrite(OBW::Ack(decoder::varint_u32())),
            _ => ()
        }

        rv
    }
}



#[test]
fn test_block_read_codec() {
    use util::test::*;
    use bytes::Bytes;

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

    let mut c = DtCodec::OpBlockRead(OBR::new());
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
        r.unwrap(),
        Some(DtR::ReadBlockResponse(pb_cons!(BlockOpResponseProto,
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
        r.unwrap(),
        Some(
            DtR::Packet(BlockDataPacket {
                header: pb_cons!(PacketHeaderProto,
                    offset_in_block: 0,
                    seqno: 0,
                    last_packet_in_block: false,
                    data_len: 5
                ),
                checksum: Bytes::from(vec![0xa9, 0xc7, 0xc0, 0x1b]),
                data: Bytes::from("ABCD\n")
            })
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


#[test]
fn test_block_write_codec() {
    use util::test::*;
    use bytes::Buf;
    use std::io::Cursor;

    //header {
    // baseHeader {
    //  block {
    //   poolId: "BP-1914853243-127.0.0.1-1500467607052"
    //   blockId: 1073742870
    //   generationStamp: 2054
    //   numBytes: 134217728
    //  }
    //  token {
    //   identifier: ""
    //   password: ""
    //   kind: ""
    //   service: ""
    //  }
    // }
    // clientName: "DFSClient_NONMAPREDUCE_-484373823_1"
    //}
    //stage: PIPELINE_SETUP_CREATE
    //pipelineSize: 1
    //minBytesRcvd: 0
    //maxBytesRcvd: 0
    //latestGenerationStamp: 0
    //requestedChecksum {type: CHECKSUM_CRC32C bytesPerChecksum: 512}
    //cachingStrategy {}
    //storageType: DISK
    //allowLazyPersist: false

    /*
    00 1c 50 5d 0a 5b 0a 34 0a 32 0a 25 42 50 2d 31 39 31 34 38 35 33 32 34 33 2d 31 32 37
    2e 30
    */
    let bs = "\
    00:1c:50:85:01:0a:68:0a:41:0a:35:0a:25:42:50:2d:31:39:31:34:38:35:33:32:34:33:2d:31:32:37:\
    2e:30:2e:30:2e:31:2d:31:35:30:30:34:36:37:36:30:37:30:35:32:10:96:88:80:80:04:18:86:10:20:\
    80:80:80:40:12:08:0a:00:12:00:1a:00:22:00:12:23:44:46:53:43:6c:69:65:6e:74:5f:4e:4f:4e:4d:\
    41:50:52:45:44:55:43:45:5f:2d:34:38:34:33:37:33:38:32:33:5f:31:20:06:28:01:30:00:38:00:40:\
    00:4a:05:08:02:10:80:04:52:00:58:01:68:00:70:00:78:00"
        .to_bytes();

    let _ = "\
    00 1c 50 75 0a 5e 0a 37 0a 35 0a 25 42 50 2d 31 39 31 34 38 35 33 32 34 33 2d 31 32 37 2e 30
    2e 30 2e 31 2d 31 35 30 30 34 36 37 36 30 37 30 35 32 10 96 88 80 80 04 18 86 10 20 80 80 80
    40 12 23 44 46 53 43 6c 69 65 6e 74 5f 4e 4f 4e 4d 41 50 52 45 44 55 43 45 5f 2d 34 38 34 33
    37 33 38 32 33 5f 31 20 06 28 01 30 00 38 00 40 00 4a 05 08 02 10 80 04 58 01 68 00
    ";

    fn dt_parse_serverside<T: PduDes + std::fmt::Debug>(b: Vec<u8>, op: Op) -> T {
        let mut bx = Cursor::new(b);
        assert_eq!(bx.get_u16::<BigEndian>(), DATA_TRANSFER_VERSION);
        assert_eq!(bx.get_u8(), op as u8);
        let mut bm = BytesMut::from(bx.into_inner().split_off(3));
        let mut dec: PduDecoder<VarIntU32Decoder, T> = decoder::varint_u32();
        let result: Result<Option<T>> = dec.decode(&mut bm);
        result.unwrap().unwrap()
    }

    let _ = dt_parse_serverside::<OpWriteBlockProto>(bs, Op::WriteBlock);
    //println!("R: {:?}", _);

    let mut c = DtCodec::new();
    let mut b = BytesMut::new();
    let owbp = pb_cons!(OpWriteBlockProto,
        header: pb_cons!(ClientOperationHeaderProto,
            base_header: pb_cons!(BaseHeaderProto,
                block: pb_cons!(ExtendedBlockProto,
                    pool_id: "BP-1914853243-127.0.0.1-1500467607052".to_owned(),
                    block_id: 1073742870,
                    generation_stamp: 2054,
                    num_bytes: 134217728
                )
            ),
            client_name: "DFSClient_NONMAPREDUCE_-484373823_1".to_owned()
        ),
        stage: OpWriteBlockProto_BlockConstructionStage::PIPELINE_SETUP_CREATE,
        pipeline_size: 1,
        min_bytes_rcvd: 0,
        max_bytes_rcvd: 0,
        latest_generation_stamp: 0,
        requested_checksum: pb_cons!(ChecksumProto,
           type: ChecksumTypeProto::CHECKSUM_CRC32C,
            bytes_per_checksum: 512
        ),
        storage_type: StorageTypeProto::DISK,
        allow_lazy_persist: false
    );
    c.encode(DtQ::WriteBlock(owbp), &mut b).unwrap();

    println!("{:?}", HexSlice(&b[..]));
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

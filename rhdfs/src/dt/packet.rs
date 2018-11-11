//! Data transfer packet implementation
//! General packet format
//! ```text
//! +-----------------------------------------------------------+
//! |  uint32 length of (this field + checksums + data)         |
//! +-----------------------------------------------------------+
//! |  uint16 length + PacketHeaderProto                        |
//! +-----------------------------------------------------------+
//! |  checksums (variable length)                              |
//! +-----------------------------------------------------------+
//! |  data                                                     |
//! +-----------------------------------------------------------+
//! ```
//! See `org.apache.hadoop.hdfs.server.datanode.BlockSender.sendPacket`
//! See `org.apache.hadoop.hdfs.protocol.datatransfer.PacketReceiver.doRead`
//! See `org.apache.hadoop.hdfs.protocol.datatransfer.PacketHeader`


use tokio_io::codec::{Decoder, Encoder};
use bytes::{BytesMut, Bytes, BufMut};
use byteorder::{ByteOrder, BigEndian};

use protobuf_api::PacketHeaderProto;
use codec_tools::*;
use ::*;

// TODO: redefine Debug so only initial bytes of BytesMut are printed, and printed in hex
/// Data packet for block read|write. Note that `header.data_len == data.len()`
#[derive(Debug, PartialEq, Clone)]
pub struct BlockDataPacket {
    pub header: PacketHeaderProto,
    pub checksum: Bytes,
    pub data: Bytes
}

impl BlockDataPacket {
    pub fn is_last(&self) -> bool { self.header.get_lastPacketInBlock() || self.header.get_dataLen() == 0 }
    pub fn seq_no(&self) -> i64 { self.header.get_seqno() }
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

impl PduSer for PacketLengths {
    fn serialized_len(&mut self) -> usize {
        Self::LEN
    }

    fn encode(self, b: &mut BytesMut) -> Result<()> {
        b.put_u32::<BigEndian>(self.other);
        b.put_u16::<BigEndian>(self.header);
        Ok(())
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
            Ok(Some(BlockDataPacket { header, checksum: checksum.freeze(), data: data.freeze() }))
        }
    }
}

#[derive(Debug)]
pub struct PacketDecoder {
    inner: PairDecoder<FixedSizeDecoder<PacketLengths>, PacketBodyDecoder>
}

impl PacketDecoder {
    pub fn new() -> PacketDecoder {
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

struct PacketBodyEncoder;

impl Encoder for PacketBodyEncoder {
    type Item = BlockDataPacket;
    type Error = Error;

    fn encode(&mut self, item: BlockDataPacket, dst: &mut BytesMut) -> Result<()> {
        let _ = item.header.encode(dst)?;
        dst.extend_from_slice(&item.checksum);
        dst.extend_from_slice(&item.data);
        Ok(())
    }
}


pub struct PacketEncoder {
    inner: PairEncoder<FixedSizeEncoder<PacketLengths>, PacketBodyEncoder>
}

impl PacketEncoder {
    pub fn new() -> PacketEncoder {
        PacketEncoder {
            inner:
            PairEncoder::new(
                FixedSizeEncoder::new(),
                PacketBodyEncoder,
                | pb | {
                    Ok(PacketLengths {
                        other: checked_usize_to_u32(pb.checksum.len() + pb.data.len() + U32_BYTES, "other")?,
                        header: checked_usize_to_u16(pb.header.serialized_len(), "header")?
                    })
                }
            )
        }
    }
}

impl Encoder for PacketEncoder {
    type Item = BlockDataPacket;
    type Error = Error;

    fn encode(&mut self, item: BlockDataPacket, dst: &mut BytesMut) -> Result<()> {
        self.inner.encode(item, dst)
    }
}


/// loopback-test encoder via decoder. Correctness  of decoder is proven in codec tests
#[test]
fn test_encoder() {
    let checksum = [b'a', b'b', b'c'];
    let data = [10u8, 11, 12, 13, 14, 15];
    let header = pb_cons!(PacketHeaderProto,
        offset_in_block: 100,
        seqno: 200,
        last_packet_in_block: false,
        data_len: data.len() as i32
        );

    let bdp = BlockDataPacket {
        header,
        checksum: Bytes::from(&checksum[..]),
        data: Bytes::from(&data[..])
    };

    let mut e = PacketEncoder::new();
    let mut b = BytesMut::new();

    let _ = e.encode(bdp, &mut b).expect("encoding failed");

    let mut d = PacketDecoder::new();

    let BlockDataPacket {
        header: o_header,
        checksum: o_checksum,
        data: o_data
    } = d.decode(&mut b).expect("decoding failed").unwrap();

    let (offset_in_block, seqno, last_packet_in_block, data_len) =
        pb_decons!(PacketHeaderProto, o_header,
        offset_in_block, seqno, last_packet_in_block, data_len);

    assert_eq!(offset_in_block, 100);
    assert_eq!(seqno, 200);
    assert_eq!(last_packet_in_block, false);
    assert_eq!(data_len, data.len() as i32);


    assert_eq!(&o_checksum[..], &checksum[..]);
    assert_eq!(&o_data[..], &data[..]);

}
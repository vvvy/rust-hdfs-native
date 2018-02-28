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


use tokio_io::codec::{Decoder/*, Encoder*/};
use bytes::{BytesMut/*, BufMut*/};
use byteorder::{ByteOrder, BigEndian};

use protobuf_api::PacketHeaderProto;
use codec_tools::*;
use ::*;

// TODO: redefine Debug so only initial bytes of BytesMut are printed, and printed in hex
#[derive(Debug, PartialEq)]
pub struct BlockDataPacket {
    pub header: PacketHeaderProto,
    pub checksum: BytesMut,
    pub data: BytesMut
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


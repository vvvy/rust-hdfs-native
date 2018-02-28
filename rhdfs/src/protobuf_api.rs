use std::io::Write;
use protobuf::{CodedOutputStream, Message, MessageStatic};
use codec_tools::*;
use bytes::{BytesMut, BufMut};
use ::*;



impl<M> PduSer for M where M: Message {
    fn serialized_len(&mut self) -> usize {
        self.compute_size() as usize
    }
    fn encode(self, b: &mut BytesMut) -> Result<()> {
        let mut w = b.writer();
        {
            let mut os = CodedOutputStream::new(&mut w);
            self.write_to_with_cached_sizes(&mut os)?;
            os.flush()?;
        }
        w.flush()?;
        Ok(())
    }
}

impl<M> PduDes for M where M: MessageStatic {
    fn from_bytes(b: BytesMut) -> Result<Self> {
        let mut o = Self::new();
        o.merge_from_bytes(&b)?;
        Ok(o)
    }
}


#[test]
fn test_pdu_ser_for_message() {
    use protocolpb::proto::datatransfer::*;
    use protocolpb::proto::hdfs::*;

    let mut b =  BytesMut::with_capacity(1024);
    let mut orbp = OpReadBlockProto::new();
    orbp.set_len(1000);
    orbp.set_offset(0);
    let mut h = ClientOperationHeaderProto::new();
    let mut bh = BaseHeaderProto::new();
    let mut ebp = ExtendedBlockProto::new();
    ebp.set_blockId(0xffbbffbb);
    ebp.set_generationStamp(0);
    ebp.set_numBytes(1000);
    ebp.set_poolId("abc".to_owned());
    bh.set_block(ebp);
    //bh.set_token()
    h.set_baseHeader(bh);
    h.set_clientName("abc".to_owned());
    orbp.set_header(h);

    let len = orbp.serialized_len();
    assert_eq!(len, 32);
    orbp.encode(&mut b).unwrap();
    assert_eq!(len, b.len());
    assert_eq!(b, BytesMut::from(vec![
        10, 25, 10, 18, 10, 16, 10, 3, 97, 98, 99, 16, 187, 255, 239, 253, 15, 24,
        0, 32, 232, 7, 18, 3, 97, 98, 99, 16, 0, 24, 232, 7
    ]));
}

pub use protocolpb::proto::datatransfer::{
    OpReadBlockProto,
    BlockOpResponseProto,
    Status as DtStatus,
    ClientOperationHeaderProto,
    BaseHeaderProto,
    ClientReadStatusProto,
    ReadOpChecksumInfoProto,
    ChecksumProto,
    PacketHeaderProto
};

pub use protocolpb::proto::hdfs::{
    ExtendedBlockProto,
    ChecksumTypeProto
};

/// Protobuf API abstraction layer.
/// Each PDU type field used must be declared here
macro_rules! pbdb {
{ OpReadBlockProto, header, $a:tt } => { pbdbf!{ get_header, set_header, $a } };
{ OpReadBlockProto, offset, $a:tt } => { pbdbf!{ get_offset, set_offset, $a } };
{ OpReadBlockProto, len, $a:tt }=> { pbdbf!{ get_len, set_len, $a } };

{ ExtendedBlockProto, pool_id, $a:tt } => { pbdbf!{ get_poolId, set_poolId, $a } };
{ ExtendedBlockProto, block_id, $a:tt } => { pbdbf!{ get_blockId, set_blockId, $a } };
{ ExtendedBlockProto, generation_stamp, $a:tt } => { pbdbf!{ get_generationStamp, set_generationStamp, $a } };
{ ExtendedBlockProto, num_bytes, $a:tt } => { pbdbf!{ get_numBytes, set_numBytes, $a } };

{ BlockOpResponseProto, status, $a:tt } => { pbdbf!{ get_status, set_status, $a } };
{ BlockOpResponseProto, read_op_checksum_info, $a:tt } => { pbdbf!{ get_readOpChecksumInfo, set_readOpChecksumInfo, $a } };
{ BlockOpResponseProto, message, $a:tt } => { pbdbf!{ get_message, set_message, $a } };

{ ClientOperationHeaderProto, client_name, $a:tt} => { pbdbf!{ get_clientName, set_clientName, $a } };
{ ClientOperationHeaderProto, base_header, $a:tt} => { pbdbf!{ get_baseHeader, set_baseHeader, $a } };

{ PacketHeaderProto, offset_in_block, $a:tt} => { pbdbf!{ get_offsetInBlock, set_offsetInBlock, $a } };
{ PacketHeaderProto, seqno, $a:tt} => { pbdbf!{ get_seqno, set_seqno, $a } };
{ PacketHeaderProto, last_packet_in_block, $a:tt} => { pbdbf!{ get_lastPacketInBlock, set_lastPacketInBlock, $a } };
{ PacketHeaderProto, data_len, $a:tt} => { pbdbf!{ get_dataLen, set_dataLen, $a } };

{ BaseHeaderProto, block, $a:tt} => { pbdbf!{ get_block, set_block, $a } };

{ ClientReadStatusProto, status, $a:tt } => { pbdbf!{ get_status, set_status, $a } };

{ ReadOpChecksumInfoProto, checksum, $a:tt } => { pbdbf!{ get_checksum, set_checksum, $a } };
{ ReadOpChecksumInfoProto, chunk_offset, $a:tt } => { pbdbf!{ get_chunkOffset, set_chunkOffset, $a } };

{ ChecksumProto, type, $a:tt } => { pbdbf!{ get_field_type, set_field_type, $a } };
{ ChecksumProto, bytes_per_checksum, $a:tt } => { pbdbf!{ get_bytesPerChecksum, set_bytesPerChecksum, $a } };

}

macro_rules! pbdbf {
    { $get:ident, $set:ident, { $r:expr } } => { $r.$get() };
    { $get:ident, $set:ident, { $r:expr, $v:expr } } => { $r.$set($v) };
}

#[macro_export]
macro_rules! pb_cons {
    { $t:ident, $($f:ident : $v:expr),* } => {
        {
            let mut r = <$t>::new();
            $(
                pbdb!{$t, $f, {r, $v}}
            )*
            r
        }
    }
}

#[macro_export]
macro_rules! pb_decons {
    { $t:ident, $r:expr, $($f:ident),+ } => {
        (
            $(
                pbdb!{$t, $f, {$r} }
            ),+
        )
    }
}

#[test]
fn test_pbapi() {
    use protocolpb::proto::hdfs::ExtendedBlockProto;
    let ebp = pb_cons!(ExtendedBlockProto,
          pool_id: "abs".to_owned(),
          block_id: 123,
          generation_stamp: 345
    );

    let (p, b, g) = pb_decons!(ExtendedBlockProto, ebp, pool_id, block_id, generation_stamp);
    assert_eq!(p, "abs".to_owned());
    assert_eq!(b, 123);
    assert_eq!(g, 345);
}


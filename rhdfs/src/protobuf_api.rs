use std::io::Write;
use protobuf::{CodedOutputStream, CodedInputStream, Message, MessageStatic, ProtobufResult};
use codec_tools::*;
use bytes::{Bytes, BytesMut, BufMut};
use ::*;

use protocolpb::proto::datatransfer::*;
use protocolpb::proto::hdfs::*;

impl<M> PduSer for M where M: Message {
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
    fn serialized_len(&mut self) -> usize {
        self.compute_size() as usize
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


//#[macro_export]
macro_rules! pb_set {
    { poolId $r:expr, $v:expr } => { $r.set_poolId($v) };
    { blockId $r:expr, $v:expr } => { $r.set_blockId($v) };
    { generationStamp $r:expr, $v:expr } => { $r.set_generationStamp($v) };
}

//#[macro_export]
macro_rules! pb_get {
    { poolId $r:expr } => { $r.get_poolId() };
    { blockId $r:expr } => { $r.get_blockId() };
    { generationStamp $r:expr } => { $r.get_generationStamp() };
}



#[macro_export]
macro_rules! pb_cons {
    { $pdu_t:path, $($field:ident = $value:expr),* } => { {
        let mut r = <$pdu_t>::new();
        $(
            pb_set!($field r, $value);
        )*
        r
    } }
}

#[macro_export]
macro_rules! pb_decons {
    { $pdu:expr, $($field:ident),* } => { (
        $(
            pb_get!($field $pdu)
        ),*
    ) }
}

fn test_pbapi() {
    let ebp = pb_cons!(ExtendedBlockProto,
          poolId = "abs".to_owned(),
          blockId = 123,
          generationStamp = 345
    );

    let (p, b, g) = pb_decons!(ebp, poolId, blockId, generationStamp);
}
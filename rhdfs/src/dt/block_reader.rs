
use std::io::Write;
use std::fmt;
use std::fmt::Debug;

use byteorder::{ByteOrder, BigEndian};
use crc::crc32;

use super::codec::{DtReq, DtRsp, OpBlockReadMessage};
use super::packet::BlockDataPacket;
use super::*;
use protobuf_api::*;
use *;

//--------------------------------------------------------------------------------------------------

trait ChecksumValidator: Send + Debug {
    fn is_trivial(&self) -> bool;
    fn is_checksum_ok(&self, data: &[u8], sums: &[u8]) -> bool;
    //fn eval(&self, data: &[u8], sums: &mut [u8]);
}

#[derive(Debug)]
struct CVTrivial;

impl ChecksumValidator for CVTrivial {
    fn is_trivial(&self) -> bool { true }
    fn is_checksum_ok(&self, _data: &[u8], _sums: &[u8]) -> bool {
        true
    }
    //fn eval(&self, data: &[u8], sums: &mut [u8]) { }
}

struct CVCRC32 {
    bytes_per_checksum: usize,
    algo: fn(&[u8]) -> u32
}

impl Debug for CVCRC32 {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        fmt.debug_struct("CVCRC32")
            .field("bytes_per_checksum", &self.bytes_per_checksum)
            .finish()
    }
}

impl ChecksumValidator for CVCRC32 {
    fn is_trivial(&self) -> bool { false }
    fn is_checksum_ok(&self, data: &[u8], sums: &[u8]) -> bool {
        let idata = data.chunks(self.bytes_per_checksum);
        let isums = sums.chunks(4); //size of checksum
        idata
            .zip(isums)
            .find(|&(d, s)| (self.algo)(d) != BigEndian::read_u32(s))
            .is_none()
    }
    /*fn eval(&self, data: &[u8], sums: &mut [u8]) {
        let idata = data.chunks(self.bytes_per_checksum);
        let b = BytesMut::f
    }*/
}

impl CVCRC32 {
    fn new_crc32(bytes_per_checksum: usize) -> CVCRC32 {
        CVCRC32 { bytes_per_checksum, algo: crc32::checksum_ieee }
    }
    fn new_crc32c(bytes_per_checksum: usize) -> CVCRC32 {
        CVCRC32 { bytes_per_checksum, algo: crc32::checksum_castagnoli }
    }
}

//--------------------------------------------------------------------------------------------------
//TODO: error handling
// Error value in the case of an error should indicate either recoverable or irrecoverable error
// Any local (W-related) error is irrecoverable.
// Remote (DN-related) errors can typically be fixed by switching to the next DN in the chain, so
// they are recoverable.
// When a failed BRT object is returned, its offset field should correctly indicate the number of bytes
// successfully read, so the read of the next DN starts at this point.
// (NEEDS ENHANCEMENT) Remote errors must terminate the DN connection early (???).
// Some remote errors (e g failed CRC) must lead to the DN being included in the failed DN list (???).
// All remote errors raised in BRT are permanent (???)


/// State of block reader
#[derive(Debug)]
pub struct BlockReadState<W: Write> {
    pub w: W,
    ///Read position inside the block: the position read so far, not including any bad data
    pub read_position: i64
}


impl<W: Write> BlockReadState<W> {
    fn new(w: W, read_position: i64) -> BlockReadState<W> {
        BlockReadState { w, read_position }
    }

}


/// A tracking structure for OpBlockRead.
/// This implementation fails fast on any error, aborting the DN connection
/// See `org.apache.hadoop.hdfs.RemoteBlockReader2`.

pub struct BlockReadTracker<W: Write> {
    w: W,
    c: Box<ChecksumValidator>,
    seqno: i64,
    offset: i64
}

impl<W: Write> Debug for BlockReadTracker<W> {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        fmt.debug_struct("BlockReadTracker")
            .field("c", &self.c)
            .field("seqno", &self.seqno)
            .field("offset", &self.offset)
            .finish()
    }
}


impl<W: Write> BlockReadTracker<W> {
    fn new(BlockReadState { w, read_position }: BlockReadState<W>, c: Box<ChecksumValidator>) -> BlockReadTracker<W> {
        BlockReadTracker {
            w, c,
            seqno: 0,
            offset: read_position
        }
    }
    fn check_sequencing(&self, seqno: i64, offset: i64) -> Result<()> {
        if self.seqno == seqno && self.offset == offset {
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
        if self.c.is_checksum_ok(data, checksums) {
            Ok(())
        } else {
            Err(app_error!(dt DtStatus::ERROR_CHECKSUM, "BlockReadTracker: checksum error"))
        }
    }

    fn write_data(&mut self, data: &[u8]) -> Result<()> {
        let w = self.w.write(&data)?;
        if w == data.len() {
            Ok(())
        } else {
            Err(app_error!(other "BlockReadTracker: short write"))
        }
    }

    fn remote_op(&mut self, p: &BlockDataPacket) -> Result<()> {
        let (seqno, offset) = pb_decons!(PacketHeaderProto, p.header, seqno, offset_in_block);
        let _ = self.check_sequencing(seqno, offset)?;
        let _ = self.validate_checksums(&p.data, &p.checksum)?;
        self.adjust_sequencing(p.data.len());
        Ok(())
    }

    fn apply(mut self, p: BlockDataPacket) -> StdResult<Self, (Error, Self)> {
        let r = self.remote_op(&p)
            .and_then(|_| self.write_data(&p.data));

        match r {
            Ok(_) => Ok(self),
            Err(e) => Err((e, self))
        }
    }

    fn decons(self) -> (DtStatus, BlockReadState<W>) {
        let a = if self.c.is_trivial() { DtStatus::SUCCESS } else { DtStatus::CHECKSUM_OK };
        let b = BlockReadState::new(self.w, self.offset);
        (a, b)
    }
}

#[derive(Debug)]
pub enum ReadBlock<W: Write + Send> {
    Init(OpReadBlockProto, BlockReadState<W>),
    ResponseWait(BlockReadState<W>),
    Packet(BlockReadTracker<W>),
    ClientReadStatusSendWait(BlockReadState<W>),
    End(BlockReadState<W>)
}


impl<W> ReadBlock<W> where
   W: Write + Send + Debug + 'static {

   pub fn new(h: BaseHeaderProto, offset: u64, len: u64, w: W) -> ReadBlock<W> {
       let orbp = pb_cons!(OpReadBlockProto,
           header: pb_cons!(ClientOperationHeaderProto,
               //client_name: client_name.to_owned(),
               base_header: h
           ),
           offset: offset,
           len: len
       );

       ReadBlock::Init(orbp,BlockReadState::new(w, offset as i64))
   }
   pub fn state(self) -> BlockReadState<W> {
       match self {
           ReadBlock::Init(_, brs) |
           ReadBlock::ClientReadStatusSendWait(brs) |
           ReadBlock::ResponseWait(brs) |
           ReadBlock::End(brs) =>
               brs,
           ReadBlock::Packet(brt) =>
               brt.decons().1
       }
   }
}

impl<W> ProtocolFsm for ReadBlock<W> where W: Write + Send + Debug + 'static {
    fn idle(self) -> (ProtocolFsmResult, ReadBlock<W>) {
        match self {
            ReadBlock::Init(orbp, w) =>
                pfsm!(send(DtReq::ReadBlock(orbp)) / goto ReadBlock::ResponseWait(w)),
            s @ ReadBlock::ResponseWait(..) |
            s @ ReadBlock::Packet(..) |
            s @ ReadBlock::End(..) =>
                pfsm!(wait / goto s),
            //pfsm!(return error(app_error!(other "Invalid s/e {:?}/idle", s))/ goto s)
            ReadBlock::ClientReadStatusSendWait(w) =>
                pfsm!(return success / goto ReadBlock::End(w))
        }
    }

    fn incoming(self, rsp: DtRsp) -> (ProtocolFsmResult, ReadBlock<W>) {
        match (self, rsp) {
            (ReadBlock::ResponseWait(brs), DtRsp::ReadBlock(OpBlockReadMessage::Initial(has_data, borp))) =>
                if has_data {
                    pfsm!(wait / goto ReadBlock::Packet(build_block_read_tracker(borp, brs)))
                } else {
                    let (s, m) = pb_decons!(BlockOpResponseProto, borp, status, message);
                    pfsm!(return error(app_error!(dt s, m)) / goto ReadBlock::End(brs))
                },
            (ReadBlock::Packet(pt), DtRsp::ReadBlock(OpBlockReadMessage::Packet(pkt))) =>
                match pt.apply(pkt) {
                    Ok(pt) => pfsm!(wait / goto ReadBlock::Packet(pt)),
                    Err((e, pt)) => pfsm!(return error(e) / goto ReadBlock::End(pt.decons().1))
                },
            (ReadBlock::Packet(pt), DtRsp::ReadBlock(OpBlockReadMessage::End)) => {
                let (s, rbs) = pt.decons();
                let crs = pb_cons!(ClientReadStatusProto, status: s);
                pfsm!(send(DtReq::ClientReadStatus(crs)) / goto ReadBlock::ClientReadStatusSendWait(rbs))
            }
            //abnormal conditions
            (ReadBlock::Init(_, brs), ev) =>
                pfsm!(return error(app_error!(other "Unexpected s/e Init/{:?}", ev)) / goto ReadBlock::End(brs)),
            (ReadBlock::Packet(pt), ev) =>
                pfsm!(return error(app_error!(other "Unexpected s/e Packet/{:?}", ev)) / goto ReadBlock::End(pt.decons().1)),
            (ReadBlock::ResponseWait(brs), ev) =>
                pfsm!(return error(app_error!(other "Unexpected s/e ResponseWait/{:?}", ev)) / goto ReadBlock::End(brs)),
            (ReadBlock::ClientReadStatusSendWait(brs), ev) =>
                pfsm!(return error(app_error!(other "Unexpected s/e ClientReadStatusSendWait/{:?}", ev)) / goto ReadBlock::End(brs)),
            (ReadBlock::End(brs), ev) =>
                pfsm!(return error(app_error!(other "Unexpected s/e End/{:?}", ev)) / goto ReadBlock::End(brs))
        }
    }
}

fn build_block_read_tracker<W: Write>(borp: BlockOpResponseProto, brs: BlockReadState<W>) -> BlockReadTracker<W> {
   let roci =
       pb_decons!(BlockOpResponseProto, borp, read_op_checksum_info);
   let (checksum, _chunk_offset) =
       pb_decons!(ReadOpChecksumInfoProto, roci, checksum, chunk_offset);
   let (ctype, bpc) =
       pb_decons!(ChecksumProto, checksum, type, bytes_per_checksum);
   let ckalg: Box<ChecksumValidator> =
       match if bpc == 0 { ChecksumTypeProto::CHECKSUM_NULL } else { ctype } {
           ChecksumTypeProto::CHECKSUM_NULL =>
               Box::new(CVTrivial),
           ChecksumTypeProto::CHECKSUM_CRC32 =>
               Box::new(CVCRC32::new_crc32(bpc as usize)),
           ChecksumTypeProto::CHECKSUM_CRC32C =>
               Box::new(CVCRC32::new_crc32c(bpc as usize))
       };
   BlockReadTracker::new(brs, ckalg)
}


#[test]
fn test_read_block_simple() {
    use util::test::ptk::*;

    //extern crate env_logger;
    //env_logger::init();

    let t = spawn_test_server("127.0.0.1:60010", test_script! {
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
    use futures::future::err;

    let addr = "127.0.0.1:60010".to_socket_addrs().unwrap().next().ok_or(app_error!(other "DN host not found")).unwrap();
    let conn = Connection::connect(&addr, "abc".to_owned());

    let h = pb_cons!(BaseHeaderProto,
                   block: pb_cons!(ExtendedBlockProto,
                       pool_id: "BP-1914853243-127.0.0.1-1500467607052".to_owned(),
                       block_id: 1073742835,
                       generation_stamp: 2015,
                       num_bytes: 5
                   )
               );

    let fsm = ReadBlock::new(h, 0, 5, vec![]);

    let fut =
        conn.then(|w| match w {
            Ok(c) => c.run(fsm),
            Err(e) => Box::new(err((e.into(), fsm)))
        });
    let (_c, rb) = fut.wait().unwrap();
    let dt::BlockReadState { w, read_position } = rb.state();
    assert_eq!(w, vec![65, 66, 67, 68, 10]);
    assert_eq!(read_position, 5);

    //-----------------------------------
    let _ = t.join().unwrap();
}
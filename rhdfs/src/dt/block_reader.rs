use std::io::ErrorKind;
use std::io::Write;
use std::fmt;
use std::fmt::Debug;

use futures::{Future};
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
}

#[derive(Debug)]
struct CVTrivial;

impl ChecksumValidator for CVTrivial {
    fn is_trivial(&self) -> bool { true }
    fn is_checksum_ok(&self, _data: &[u8], _sums: &[u8]) -> bool {
        true
    }
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

/// A tracking structure for OpBlockRead.
/// NOTE: Presumably, this implementation has error handling semantics different from
/// `org.apache.hadoop.hdfs.RemoteBlockReader2`:
/// * The latter throws and (presumably) aborts immediately upon any error, checksum or sequencing.
/// * We try to read until the end (which may be incorrect)
pub struct BlockReadTracker<W: Write> {
    w: W,
    c: Box<ChecksumValidator>,
    seqno: i64,
    offset: i64,
    remote_error: Option<Error>,
    local_error: Option<Error>
}

impl<W: Write> Debug for BlockReadTracker<W> {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        fmt.debug_struct("BlockReadTracker")
            .field("c", &self.c)
            .field("seqno", &self.seqno)
            .field("offset", &self.offset)
            .field("remote_error", &self.remote_error)
            .field("local_error", &self.local_error)
            .finish()
    }
}


impl<W: Write> BlockReadTracker<W> {
    fn new(w: W, c: Box<ChecksumValidator>) -> BlockReadTracker<W> {
        BlockReadTracker {
            w, c,
            seqno: 0,
            offset: 0,
            local_error: None,
            remote_error: None }
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


    fn apply(mut self, p: BlockDataPacket) -> Self {
        trace!(target: "dt_proto", "BlockReadTracker: packet header={:?}, dlen={}, cksumlen={}", p.header, p.data.len(), p.checksum.len());
        //if there is a remote error - do nothing, else
        //if there is a local error - do everything except for a local operation itself
        if self.remote_error.is_none() {
            if let Err(e) = self.remote_op(&p) {
                error!("BlockReadTracker: remote error at block {}: {}", self.seqno, e);
                self.remote_error = Some(e);
            } else {
                if self.local_error.is_none() {
                    if let Err(e) = self.write_data(&p.data) {
                        error!("BlockReadTracker: local error: {}", e);
                        self.local_error = Some(e);
                    }
                }
            }
        }
        self
    }

    fn decons(self) -> (DtStatus, Option<Error>, Option<Error>, W) {
        let s = match &self.remote_error {
            &Some(Error::DataTransfer(s, _)) => s,
            &Some(_) => DtStatus::ERROR,    //should never happen
            &None => //see org.apache.hadoop.hdfs.RemoteBlockReader2.readNextPacket()
                if self.c.is_trivial() { DtStatus::SUCCESS } else { DtStatus::CHECKSUM_OK }
        };
        (s, self.remote_error, self.local_error, self.w)
    }
}

pub struct ReadBlockRequest<'a> {
    pub pool_id: &'a str,
    pub block_id: u64,
    pub generation_stamp: u64,
    pub offset: u64,
    pub num_bytes: u64
}


#[derive(Debug)]
pub enum OpReadBlockFsm<W: Write + Send> {
    Null(OpReadBlockProto, W),
    ResponseWait(W),
    Packet(BlockReadTracker<W>),

    //final states
    Success(Option<Error>, Option<Error>, W),
    Error(BlockOpResponseProto, W)
}

impl<W> OpReadBlockFsm<W> where
    W: Write + Send + Debug + 'static {
    pub fn new<'a, 'b>(r: ReadBlockRequest<'a>, client_name: &'b str, w: W) -> OpReadBlockFsm<W> {
        let orbp = pb_cons!(OpReadBlockProto,
            header: pb_cons!(ClientOperationHeaderProto,
                client_name: client_name.to_owned(),
                base_header: pb_cons!(BaseHeaderProto,
                    block: pb_cons!(ExtendedBlockProto,
                        pool_id: r.pool_id.to_owned(),
                        block_id: r.block_id,
                        generation_stamp: r.generation_stamp,
                        num_bytes: r.num_bytes
                    )
                )
            ),
            offset: r.offset,
            len: r.num_bytes
        );

        OpReadBlockFsm::Null(orbp,w)
    }

    pub fn result(self) -> Result<W> {
        match self {
            OpReadBlockFsm::Success(None, None, w) =>
                Ok(w),
            OpReadBlockFsm::Success(r, l, _w) =>
                Err(r.unwrap_or_else(|| l.unwrap_or_else(|| app_error!(unreachable)))),
            OpReadBlockFsm::Error(borp, _) => {
                let (s, m) = pb_decons!(BlockOpResponseProto, borp, status, message);
                Err( app_error!(dt s, m))
            },
            other =>
                Err(app_error!(other "invalid OpReadBlockFsm terminal state: {:?}", other))
        }
    }
}

impl<W> ProtocolFsm for OpReadBlockFsm<W> where W: Write + Send + Debug {
    fn idle(self) -> (ProtocolFsmResult, OpReadBlockFsm<W>) {
        match self {
            OpReadBlockFsm::Null(orbp, w) =>
                pfsm!(send(DtReq::ReadBlock(orbp)) / goto OpReadBlockFsm::ResponseWait(w)),
            s @ OpReadBlockFsm::ResponseWait(..) =>
                pfsm!(wait/ goto s),
            s @ OpReadBlockFsm::Packet(..) =>
                pfsm!(wait/ goto s),
            s @ OpReadBlockFsm::Success(..) =>
                pfsm!(return result s),
            s =>
                pfsm!(invalid ("<idle event>") in (s))
        }
    }

    fn incoming(self, rsp: DtRsp) -> (ProtocolFsmResult, OpReadBlockFsm<W>) {
        match (self, rsp) {
            (OpReadBlockFsm::ResponseWait(w), DtRsp::ReadBlock(OpBlockReadMessage::Initial(has_data, borp))) =>
                if has_data {
                    pfsm!(wait / goto OpReadBlockFsm::Packet(build_block_read_tracker(borp, w)))
                } else {
                    pfsm!(return result OpReadBlockFsm::Error(borp, w))
                },
            (OpReadBlockFsm::Packet(pt), DtRsp::ReadBlock(OpBlockReadMessage::Packet(pkt))) =>
                pfsm!(wait / goto OpReadBlockFsm::Packet(pt.apply(pkt))),
            (OpReadBlockFsm::Packet(pt), DtRsp::ReadBlock(OpBlockReadMessage::End)) => {
                let (s, re, le, w) = pt.decons();
                pfsm!(
                    send(DtReq::ClientReadStatus(pb_cons!(ClientReadStatusProto, status: s))) /
                        goto OpReadBlockFsm::Success(re, le, w))
            },
            (s, e) => pfsm!(invalid (e) in (s))
        }
    }
}


fn build_block_read_tracker<W: Write>(borp: BlockOpResponseProto, w: W) -> BlockReadTracker<W> {
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
    BlockReadTracker::new(w, ckalg)
}

pub fn read_block<'a, W: Write + Send + Debug + 'static>(c: Connection, r: ReadBlockRequest<'a>, w: W)
                                                         -> BFI<(Result<W>, Connection)> {
    let fsm = OpReadBlockFsm::new(r, c.client_name(), w);
    Box::new(c.run(fsm).map(|(c, p)|(p.result(), c))
    )
}


#[test]
fn test_read_block_simple() {
    use util::test::ptk::*;

    //extern crate env_logger;
    //env_logger::init();


    /*
        use bytes::BytesMut;
        use codec_tools::PduSer;
        use tokio_io::codec::Encoder;
        let crsp = pb_cons!(ClientReadStatusProto, status: DtStatus::CHECKSUM_OK);
        let mut m = BytesMut::new();
        //crsp.encode(&mut m);
        codec_tools::encoder::varint_u32().encode(crsp, &mut m);
        println!("@@@@@ {:?}", CDebug(&m));
    */

    let t = spawn_test_server("127.0.0.1:60010", test_script!{
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

    let addr = "127.0.0.1:60010".to_socket_addrs().unwrap().next().ok_or(app_error!(other "DN host not found")).unwrap();
    let conn = Connection::connect(&addr, "abc".to_owned());

    let arg = ReadBlockRequest {
        pool_id: "BP-1914853243-127.0.0.1-1500467607052",
        block_id: 1073742835,
        generation_stamp: 2015,
        offset: 0,
        num_bytes: 5
    };

    let fut = conn.and_then(|c| read_block(c, arg, vec![]));
    let r = fut.wait();

    assert_eq!(r.ok().and_then(|w| w.0.ok()), Some(vec![65, 66, 67, 68, 10]));

    //-----------------------------------
    let _ = t.join().unwrap();
}

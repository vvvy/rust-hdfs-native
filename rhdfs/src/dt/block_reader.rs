
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
/*
pub struct ReadBlockRequest<'a> {
    pub pool_id: &'a str,
    pub block_id: u64,
    pub generation_stamp: u64,
    pub offset: u64,
    pub num_bytes: u64
}
*/

#[derive(Debug)]
pub enum ReadBlock<W: Write + Send> {
    Null(OpReadBlockProto, W),
    ResponseWait(W),
    Packet(BlockReadTracker<W>),

    //final states
    Success(Option<Error>, Option<Error>, W),
    ServerError(BlockOpResponseProto, W),
    OtherError(IoError, W)
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

        ReadBlock::Null(orbp,w)
    }

    pub fn extract_resources(self) -> W {
        match self {
            ReadBlock::Null(_, w) |
            ReadBlock::ResponseWait(w) |
            ReadBlock::Packet(BlockReadTracker { w, .. }) |
            ReadBlock::Success(_, _, w) |
            ReadBlock::ServerError(_, w) |
            ReadBlock::OtherError(_, w) =>
                w
        }
    }

    #[inline]
    fn errvec(e1: Option<Error>, e2: Option<Error>, e3: Option<Error>) -> Vec<Error> {
        let mut rv = Vec::new();
        if let Some(e) = e1  { rv.push(e) }
        if let Some(e) = e2  { rv.push(e) }
        if let Some(e) = e3  { rv.push(e) }
        rv
    }

    pub fn result(self) -> (W, Vec<Error>) {
        match self {
            ReadBlock::Success(r, l, w) =>
                (w, Self::errvec(r, l, None)),
            ReadBlock::ServerError(borp, w) => {
                let (s, m) = pb_decons!(BlockOpResponseProto, borp, status, message);
                (w, vec![app_error!(dt s, m)])
            },
            ReadBlock::OtherError(e, w) =>
                (w, vec![e.into()]),
            ReadBlock::Packet(BlockReadTracker { w, remote_error, local_error, .. }) =>
                (w, Self::errvec(remote_error, local_error, Some(app_error!(other "invalid ReadBlock terminal state: Packet")))),
            ReadBlock::ResponseWait(w) =>
                (w, vec![app_error!(other "invalid ReadBlock terminal state: ResponseWait")]),
            ReadBlock::Null(_, w) =>
                (w, vec![app_error!(other "invalid ReadBlock terminal state: Null")])
        }
    }
}

impl<W> ProtocolFsm for ReadBlock<W> where W: Write + Send + Debug + 'static {
    fn idle(self) -> (ProtocolFsmResult, ReadBlock<W>) {
        match self {
            ReadBlock::Null(orbp, w) =>
                pfsm!(send(DtReq::ReadBlock(orbp)) / goto ReadBlock::ResponseWait(w)),
            s @ ReadBlock::ResponseWait(..) =>
                pfsm!(wait/ goto s),
            s @ ReadBlock::Packet(..) =>
                pfsm!(wait/ goto s),
            s @ ReadBlock::Success(..) =>
                pfsm!(exit s),
            s @ ReadBlock::ServerError(..) | s @ ReadBlock::OtherError(..) =>
                pfsm!(exit s)
        }
    }

    fn incoming(self, rsp: DtRsp) -> (ProtocolFsmResult, ReadBlock<W>) {
        match (self, rsp) {
            (ReadBlock::ResponseWait(w), DtRsp::ReadBlock(OpBlockReadMessage::Initial(has_data, borp))) =>
                if has_data {
                    pfsm!(wait / goto ReadBlock::Packet(build_block_read_tracker(borp, w)))
                } else {
                    pfsm!(exit ReadBlock::ServerError(borp, w))
                },
            (ReadBlock::Packet(pt), DtRsp::ReadBlock(OpBlockReadMessage::Packet(pkt))) =>
                pfsm!(wait / goto ReadBlock::Packet(pt.apply(pkt))),
            (ReadBlock::Packet(pt), DtRsp::ReadBlock(OpBlockReadMessage::End)) => {
                let (s, re, le, w) = pt.decons();
                pfsm!(
                    send(DtReq::ClientReadStatus(pb_cons!(ClientReadStatusProto, status: s))) /
                        goto ReadBlock::Success(re, le, w))
            },
            //abnormal conditions
            (s, e) =>
                pfsm!(exit ReadBlock::OtherError(
                    app_error!(other "Unexpected s/e {:?}/{:?}", s, e).into(),
                    s.extract_resources()
                ))
        }
    }
}

impl<W> ErrorAccumulator for ReadBlock<W> where W: Write + Send + Debug +'static {
    fn error(self, e: IoError) -> Self { ReadBlock::OtherError(e, self.extract_resources()) }
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

pub fn read_block<W: Write + Send + Debug + 'static>(
    c: Connection, h: BaseHeaderProto, offset: u64, len: u64, w: W
) -> BF<((W, Vec<Error>), Connection), ReadBlock<W>> {
    let fsm = ReadBlock::new(h, offset, len, w);
    Box::new(c.run(fsm).map(|(c, p)|(p.result(), c)))
}


#[test]
fn test_read_block_simple() {
    use util::test::ptk::*;

    //extern crate env_logger;
    //env_logger::init();

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
            Err(e) => b_accumulate(fsm, e)//Box::new(err(fsm.error(e)))
        });
    let r = fut.wait();

    let (w, errs) = r.map(|(_conn, p)| p.result()).unwrap();
    assert_eq!(w, vec![65, 66, 67, 68, 10]);
    assert!(errs.is_empty());

    //-----------------------------------
    let _ = t.join().unwrap();
}
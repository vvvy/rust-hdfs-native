use *;
use bytes::*;
use super::{
    packet::BlockDataPacket,
    checksum::{ChecksumValidator}
};

pub type ReadRange = (i64, i64);

#[derive(Debug)]
pub struct ReadStreamer {
    checksum: Box<ChecksumValidator>,
    read_range: ReadRange,
    seqno: i64,
    offset: i64
}

impl ReadStreamer {
    pub fn new(read_range: ReadRange, checksum: Box<ChecksumValidator>) -> ReadStreamer {
        ReadStreamer {
            checksum,
            read_range,
            seqno: 0,
            offset: 0,
        }
    }

    fn check_sequencing(&mut self, seqno: i64, offset: i64) -> Result<()> {
        if self.seqno == seqno && self.offset == offset {
            Ok(())
        } else if self.seqno == seqno && seqno == 0 {
            self.offset = offset;
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
        if self.checksum.is_checksum_ok(data, checksums) {
            Ok(())
        } else {
            Err(app_error!(dt DtStatus::ERROR_CHECKSUM, "BlockReadTracker: checksum error"))
        }
    }

    pub fn process_packet(&mut self, p: BlockDataPacket) -> Result<Bytes> {
        let (seqno, offset) = pb_decons!(PacketHeaderProto, p.header, seqno, offset_in_block);
        let _ = self.check_sequencing(seqno, offset)?;
        let _ = self.validate_checksums(&p.data, &p.checksum)?;
        self.adjust_sequencing(p.data.len());
        Ok(crop_bytes(p.data.freeze(), offset, self.read_range))
    }

    pub fn get_success_status(&self) -> DtStatus {
        if self.checksum.is_trivial() { DtStatus::SUCCESS } else { DtStatus::CHECKSUM_OK }
    }
}

/// Crops `Bytes` so that it fits into a target range.
/// `bs` is source `Bytes` where the source segment `[sl, sl + bs.len())` map, and `[rl, ru)`
/// is the target (bounding) range. Returns cropped `bs` (a result of intersection of the source
/// segment and the target range). The return value is empty if they do not intersect.
fn crop_bytes(mut b: Bytes, sl: i64, (rl, ru): (i64, i64)) -> Bytes {
    let su = sl + b.len() as i64;
    if su <= ru { //SR
        if rl <= sl { //rsSR
            //sS
            b
        } else if su <= rl { //Sr => sSrR
            Bytes::new()
        } else { //srSR
            //rS: cut off initial r - s bytes
            b.advance((rl - sl) as usize);
            b
        }
    } else { //RS
        if ru <= sl { //Rs => rRsS
            Bytes::new()
        } else if rl <= sl { //rs => rsRS
            //sR: cut off trailing bytes to the length R - s
            b.truncate((ru - sl) as usize);
            b
        } else { //srRS
            //rR: advance by r - s bytes and truncate to R - r bytes
            b.advance((rl - sl) as usize);
            b.truncate((ru - rl) as usize);
            b
        }
    }
}

#[test]
fn test_crop_bytes() {
    let sl = 10; //,20)
    let b = Bytes::from_static(&[10, 11, 12, 13, 14, 15, 16, 17, 18, 19]);

    macro_rules! asrt {
        ($b:expr, $rlru:expr => $($n:expr),+) => { assert_eq!(crop_bytes($b.clone(), sl, $rlru), Bytes::from_static(&[$($n),+])); };
        ($b:expr, $rlru:expr => ) => { assert_eq!(crop_bytes($b.clone(), sl, $rlru), Bytes::new()); };
    }

    asrt!(b, (0, 30) => 10, 11, 12, 13, 14, 15, 16, 17, 18, 19);
    asrt!(b, (0, 20) => 10, 11, 12, 13, 14, 15, 16, 17, 18, 19);
    asrt!(b, (10, 30) => 10, 11, 12, 13, 14, 15, 16, 17, 18, 19);
    asrt!(b, (10, 20) => 10, 11, 12, 13, 14, 15, 16, 17, 18, 19);
    asrt!(b, (20, 30) => );
    asrt!(b, (0, 10) => );
    asrt!(b, (15, 25) => 15, 16, 17, 18, 19);
    asrt!(b, (5, 15) => 10, 11, 12, 13, 14);
    asrt!(b, (11, 12) => 11);
    asrt!(b, (11, 13) => 11, 12);

    let b = Bytes::from_static(&[10]);

    asrt!(b, (0, 30) => 10);
    asrt!(b, (10, 11) => 10);
    asrt!(b, (20, 30) => );
    asrt!(b, (0, 10) => );
    asrt!(b, (15, 25) => );
    asrt!(b, (5, 15) => 10);
    asrt!(b, (11, 12) => );
    asrt!(b, (11, 13) => );

    let b = Bytes::new();
    asrt!(b, (0, 10) => );

}
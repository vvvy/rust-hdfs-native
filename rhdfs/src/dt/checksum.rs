use std::fmt;
use std::fmt::Debug;
use byteorder::{ByteOrder, BigEndian};
use crc::crc32;
use protobuf_api::*;


pub trait ChecksumValidator: Send + Debug {
    fn is_trivial(&self) -> bool;
    fn is_checksum_ok(&self, data: &[u8], sums: &[u8]) -> bool;
    //fn eval(&self, data: &[u8], sums: &mut [u8]);
}

#[derive(Debug)]
pub struct CVTrivial;

impl ChecksumValidator for CVTrivial {
    fn is_trivial(&self) -> bool { true }
    fn is_checksum_ok(&self, _data: &[u8], _sums: &[u8]) -> bool {
        true
    }
    //fn eval(&self, data: &[u8], sums: &mut [u8]) { }
}

pub struct CVCRC32 {
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
    pub fn new_crc32(bytes_per_checksum: usize) -> CVCRC32 {
        CVCRC32 { bytes_per_checksum, algo: crc32::checksum_ieee }
    }
    pub fn new_crc32c(bytes_per_checksum: usize) -> CVCRC32 {
        CVCRC32 { bytes_per_checksum, algo: crc32::checksum_castagnoli }
    }
}

pub fn new_checksum(csp: ChecksumProto) -> Box<ChecksumValidator> {
    let (ctype, bpc) = pb_decons!(ChecksumProto, csp, type, bytes_per_checksum);
    match if bpc == 0 { ChecksumTypeProto::CHECKSUM_NULL } else { ctype } {
        ChecksumTypeProto::CHECKSUM_NULL =>
            Box::new(CVTrivial),
        ChecksumTypeProto::CHECKSUM_CRC32 =>
            Box::new(CVCRC32::new_crc32(bpc as usize)),
        ChecksumTypeProto::CHECKSUM_CRC32C =>
            Box::new(CVCRC32::new_crc32c(bpc as usize))
    }
}
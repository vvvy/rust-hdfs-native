#![allow(dead_code)]

use std::borrow::Cow;
use std::marker::PhantomData;
use std::fmt::Debug;

use tokio_io::codec::{Decoder, Encoder};
use bytes::{BytesMut, BufMut};
use byteorder::{ByteOrder, BigEndian};

use util::*;
use ::*;

pub const U32_BYTES: usize = 4;
pub const U16_BYTES: usize = 2;


/// PDU Deserializer
pub trait PduDes where Self: Sized {
    /// Attempts to decode a PDU from a buffer.
    /// The buffer contains exactly the PDU in its serialized form.
    fn from_bytes(b: BytesMut) -> Result<Self>;
}

/// PDU Serializer
pub trait PduSer {
    fn serialized_len(&mut self) -> usize;
    fn encode(self, b: &mut BytesMut) -> Result<()>;
}

/// Wrapper for `BytesMut`
#[derive(PartialEq, Debug)]
pub struct BytesMutW {
    b: BytesMut
}

impl BytesMutW {
    pub fn from_static(b: &'static [u8]) -> BytesMutW { BytesMutW { b: BytesMut::from(b) } }
}

impl PduDes for BytesMutW {
    fn from_bytes(b: BytesMut) -> Result<Self> { Ok(BytesMutW { b }) }
}

impl PduSer for BytesMutW {
    fn serialized_len(&mut self) -> usize {
        self.b.len()
    }

    fn encode(self, b: &mut BytesMut) -> Result<()> {
        b.put(self.b);
        Ok(())
    }
}

pub trait FixedSizePdu {
    const PDU_SIZE: usize;
}

#[derive(Debug)]
pub struct VarIntU32Decoder {
    cur: usize,
    bit: u8,
    pos: usize
}

impl VarIntU32Decoder {
    pub fn new() -> VarIntU32Decoder { VarIntU32Decoder { cur: 0, bit: 0, pos: 0 } }
}

impl Decoder for VarIntU32Decoder {
    type Item = u32;
    type Error = Error;
    fn decode(&mut self, src: &mut BytesMut) -> Result<Option<u32>> {
        loop {
            if self.pos >= src.len() {
                break Ok(None)
            } else {
                let cv = src[self.pos];
                if cv & 0x80 == 0 {
                    break if self.bit == 28 && cv > 0x0f {
                        Err(app_error! {codec "VarIntU32Decoder: overflow"})
                    } else {
                        src.advance(self.pos + 1);
                        Ok(Some(self.cur as u32 | ((cv as u32) << self.bit)))
                    }
                } else {
                    if self.bit >= 28 { break Err(app_error! {codec "VarIntU32Decoder: overflow"}) }
                    self.cur |= ((cv & 0x7f) as usize) << self.bit;
                    self.bit += 7;
                    self.pos += 1;
                }
            }
        }
    }
}

pub struct VarIntU32Encoder;

impl Encoder for VarIntU32Encoder {
    type Item = u32;
    type Error = Error;
    fn encode(&mut self, mut value: Self::Item, to: &mut BytesMut) -> Result<()> {
        to.reserve(5);
        while value >= 0x80 {
            to.put(value as u8 | 0x80);
            value >>= 7;
        }
        to.put(value as u8);
        Ok(())
    }
}

/// returns number of bytes needed to encode the argument using varint encoding
pub fn varint_encoded_len(n: u32) -> u32 {
    if n == 0 { 1 } else { (32 - n.leading_zeros() - 1)/7 + 1 }
}

/*
#[test]
fn test_varint_encoded_len() {
    use std::ops::Shl;
    for x in 1..4 {
        for n in 1u32.shl(x*7 - 1)..1u32.shl(x*7 + 1) {
            let mut b = BytesMut::with_capacity(10);
            VarIntU32Encoder.encode(n, &mut b);
            assert_eq!(b.len(), varint_encoded_len(n) as usize)
        }
    }
}
*/

/// Two PDUs in sequence, each preceded by varint length. The latter is optional
#[derive(Debug)]
pub struct BiPdu<A, B> {
    pub a: A,
    pub b: Option<B>
}

impl<A, B> BiPdu<A, B> {
    pub fn new(a: A, b: Option<B>) -> BiPdu<A, B> { BiPdu { a, b }}
    pub fn new_one(a: A) -> BiPdu<A, B> { BiPdu { a, b: None }}
    pub fn new_both(a: A, b: B) -> BiPdu<A, B> { BiPdu { a, b: Some(b) }}
}

fn force_complete<T>(r: Result<Option<T>>) -> Result<T> {
    r.and_then(|w| match w {
        Some(r) => Ok(r),
        None => Err(app_error!(codec "Partial data on decoding"))
    })
}

impl<A, B> PduDes for BiPdu<A, B> where A: PduDes + Debug, B: PduDes + Debug {
    fn from_bytes(mut src: BytesMut) -> Result<Self> {
        let a = force_complete(decoder::varint_u32().decode(&mut src))?;
        let b = if !src.is_empty() {
            Some(force_complete(decoder::varint_u32().decode(&mut src))?)
        } else {
            None
        };

        if src.is_empty() {
            Ok(BiPdu {a, b})
        } else {
            Err(app_error!(codec "Trailing garbage"))
        }
    }
}


impl<A, B> PduSer for BiPdu<A, B> where A: PduSer, B: PduSer {

    fn serialized_len(&mut self) -> usize {
        #[inline]
        fn w_len(l: usize) -> usize { l + varint_encoded_len(l as u32) as usize }

        let (la, lb) = (
            w_len(self.a.serialized_len()),
            match &mut self.b {
                &mut Some(ref mut w) => w_len(w.serialized_len()),
                &mut None => 0
            }
        );

        la + lb
    }

    fn encode(self, b: &mut BytesMut) -> Result<()> {
        let _ = encoder::varint_u32().encode(self.a, b)?;
        if let Some(w) = self.b {
            let _ = encoder::varint_u32().encode(w, b)?;
        }
        Ok(())
    }
}

/// Three PDUs in sequence, each preceded by varint length
pub struct TriPdu<A, B, C> {
    a: A,
    b: B,
    c: C
}

impl<A, B, C> TriPdu<A, B, C> {
    pub fn new(a: A, b: B, c: C) -> TriPdu<A, B, C> { TriPdu { a, b, c }}
}

impl<A, B, C> PduSer for TriPdu<A, B, C> where A: PduSer, B: PduSer, C: PduSer {
    fn serialized_len(&mut self) -> usize {
        let (la, lb, lc) = (
            self.a.serialized_len(),
            self.b.serialized_len(),
            self.c.serialized_len(),
        );

        la + lb + lc +
            varint_encoded_len(la as u32) as usize +
            varint_encoded_len(lb as u32) as usize +
            varint_encoded_len(lc as u32) as usize
    }

    fn encode(self, b: &mut BytesMut) -> Result<()> {
        let _ = encoder::varint_u32().encode(self.a, b)?;
        let _ = encoder::varint_u32().encode(self.b, b)?;
        let _ = encoder::varint_u32().encode(self.c, b)?;
        Ok(())
    }
}

/// Reads `sz` bytes, then deserializes it via `PduDes`
#[derive(Debug)]
pub struct FixedSizeDecoder<I> {
    sz: usize,
    pdu_type: PhantomData<I>
}

impl<I> FixedSizeDecoder<I> {
    pub fn new_sized(sz: usize) -> FixedSizeDecoder<I> { FixedSizeDecoder { sz, pdu_type: PhantomData } }
}

impl<I> FixedSizeDecoder<I> where I: FixedSizePdu {
    pub fn new() -> FixedSizeDecoder<I> { FixedSizeDecoder { sz: I::PDU_SIZE, pdu_type: PhantomData } }
}

impl<I> Decoder for FixedSizeDecoder<I> where I: PduDes {
    type Item = I;
    type Error = Error;

    fn decode(&mut self, src: &mut BytesMut) -> Result<Option<Self::Item>> {
        if src.len() >= self.sz {
            let pdu = src.split_to(self.sz);
            I::from_bytes(pdu).map(|w| Some(w))
        } else {
            Ok(None)
        }
    }
}

/// Generates a tail decoder using the head item just read
pub trait TailF<H, T> where H: Decoder {
    fn tail_f(&self, H::Item) -> Result<T>;
}

/// `fn` based, context-free, impl of `TailF`
#[derive(Debug)]
pub struct FnTailF<H: Decoder, T> {
    f: fn(H::Item) -> Result<T>
}

impl<H: Decoder, T> FnTailF<H, T> {
    pub fn new(f: fn(H::Item) -> Result<T>) -> FnTailF<H, T> { FnTailF { f } }
}

impl<H, T> TailF<H, T> for FnTailF<H, T> where H: Decoder {
    fn tail_f(&self, item: H::Item) -> Result<T> {
        (self.f)(item)
    }
}

/// Decodes a head item via `H`, then generates 'T' and decodes a tail item via it
#[derive(Debug)]
pub enum PairDecoder<H, T, F = FnTailF<H, T>> {
    H(H, F),
    T(T)
}

impl<H, T, F> PairDecoder<H, T, F> {
    pub fn new(h: H, f: F) -> PairDecoder<H, T, F> {
        PairDecoder::H(h, f)
    }
}

impl<H, T, F> Decoder for PairDecoder<H, T, F> where
    H: Decoder<Error=Error> + Debug,
    T: Decoder<Error=Error> + Debug,
    F: TailF<H, T> + Debug,
    T::Item: Debug
{
    type Item = T::Item;
    type Error = Error;

    fn decode(&mut self, src: &mut BytesMut) -> Result<Option<Self::Item>> {
        logging_fsm_turn("PairDecoder", self, |s| match s {
            &mut PairDecoder::H(ref mut h, ref f) =>
                match h.decode(src) {
                    Ok(Some(hi)) => match f.tail_f(hi) {
                        Ok(t) => SV::S(PairDecoder::T(t)),
                        Err(e) => SV::V(Err(e))
                    },
                    Ok(None) => SV::V(Ok(None)),
                    Err(e) => SV::V(Err(e))
                },
            &mut PairDecoder::T(ref mut t) =>
                match t.decode(src) {
                    Ok(Some(ti)) => SV::V(Ok(Some(ti))),
                    Ok(None) => SV::V(Ok(None)),
                    Err(e) => SV::V(Err(e))
                }
        })
    }
}

#[derive(Debug)]
pub struct FixedSizeDecoderTailF<HI> {
    f: fn(HI) -> usize
}

impl<H, TI> TailF<H, FixedSizeDecoder<TI>> for FixedSizeDecoderTailF<H::Item> where H: Decoder {
    fn tail_f(&self, item: H::Item) -> Result<FixedSizeDecoder<TI>> {
        Ok(FixedSizeDecoder::new_sized((self.f)(item)))
    }
}

pub type PduDecoder<H, TI> = PairDecoder<H, FixedSizeDecoder<TI>, FixedSizeDecoderTailF<<H as Decoder>::Item>>;
pub fn pdu_decoder<H, TI>(h: H, f: fn(H::Item) -> usize) -> PduDecoder<H, TI>
where
    H: Decoder,
    TI: PduDes
{
    PairDecoder::new(h, FixedSizeDecoderTailF{ f })
}

pub type PduPairDecoder<HI, TI> = PduDecoder<FixedSizeDecoder<HI>, TI>;
pub fn pdu_pair_decoder_sized<HI, TI>(head_sz: usize, f: fn(HI) -> usize) -> PduPairDecoder<HI, TI>
where
    HI: PduDes + Debug,
    TI: PduDes + Debug
{
    pdu_decoder(FixedSizeDecoder::new_sized(head_sz), f)
}
pub fn pdu_pair_decoder<HI, TI>() -> PduPairDecoder<HI, TI>
    where
        HI: PduDes + Debug + FixedSizePdu + Into<usize>,
        TI: PduDes + Debug
{
    pdu_decoder(FixedSizeDecoder::new(), |w| w.into())
}

//--------------------------------------------------------------------------------------------------
pub struct FixedSizeEncoder<U> {
    pdu_type: PhantomData<U>
}

impl<I: PduSer> FixedSizeEncoder<I> {
    pub fn new() -> FixedSizeEncoder<I> { FixedSizeEncoder { pdu_type: PhantomData } }
    pub fn serialized_len(item: &mut I) -> usize { item.serialized_len() }
}

impl<I: PduSer> Encoder for FixedSizeEncoder<I> {
    type Item = I;
    type Error = Error;

    fn encode(&mut self, item: Self::Item, dst: &mut BytesMut) -> Result<()> {
        item.encode(dst)
    }
}

pub struct PairEncoder<H, T> where H: Encoder, T: Encoder {
    h: H,
    t: T,
    f: fn(&mut T::Item) -> Result<H::Item>
}

impl <H, T> PairEncoder<H, T> where H: Encoder, T: Encoder {
    pub fn new(h: H, t: T, f: fn(&mut T::Item) -> Result<H::Item>) -> PairEncoder<H, T> {
        PairEncoder { h, t, f }
    }
}

impl<H, T> Encoder for PairEncoder<H, T> where
    H: Encoder<Error=Error>,
    T: Encoder<Error=Error>
{
    type Item = T::Item;
    type Error = Error;

    fn encode(&mut self, mut item: Self::Item, dst: &mut BytesMut) -> Result<()> {
        let hi = (self.f)(&mut item)?;
        let _ = self.h.encode(hi, dst)?;
        self.t.encode(item, dst)
    }
}

pub struct PduEncoder<H, TI> where H: Encoder {
    h: H,
    t: PhantomData<TI>,
    f: fn(usize) -> Result<H::Item>
}

impl<H, TI> PduEncoder<H, TI> where H: Encoder {
    pub fn new(h: H, f: fn(usize) -> Result<H::Item>) -> PduEncoder<H, TI> { PduEncoder { h, f, t: PhantomData } }
}

impl<H, TI> Encoder for PduEncoder<H, TI> where
    H: Encoder<Error=Error>,
    TI: PduSer
{
    type Item = TI;
    type Error = Error;

    fn encode(&mut self, mut item: Self::Item, dst: &mut BytesMut) -> Result<()> {
        let hi = (self.f)(item.serialized_len())?;
        let _ = self.h.encode(hi, dst)?;
        item.encode(dst)
    }
}

pub struct PduPairEncoder<HI, TI> {
    h: PhantomData<HI>,
    t: PhantomData<TI>,
    f: fn(usize) -> Result<HI>
}

impl<HI, TI> Encoder for PduPairEncoder<HI, TI> where
    HI: PduSer,
    TI: PduSer
{
    type Item = TI;
    type Error = Error;

    fn encode(&mut self, mut item: Self::Item, dst: &mut BytesMut) -> Result<()> {
        (self.f)(item.serialized_len())?.encode(dst)?;
        item.encode(dst)
    }
}

#[inline]
pub fn checked_usize_to_u32(n: usize, context: &str) -> Result<u32> {
    if n <= u32::max_value() as usize {
        Ok(n as u32)
    } else {
        Err(app_error!(codec "{}: length overflow", context))
    }
}

#[inline]
pub fn checked_usize_to_u16(n: usize, context: &str) -> Result<u16> {
    if n <= u16::max_value() as usize {
        Ok(n as u16)
    } else {
        Err(app_error!(codec "{}: length overflow", context))
    }
}


pub fn elen_varint_u32(n: usize) -> Result<u32> { checked_usize_to_u32(n, "varintu32") }

pub fn elen_u32(n: usize) -> Result<U32W> {
    if n <= u32::max_value() as usize {
        Ok(U32W::from(n as u32))
    } else {
        Err(app_error!{ codec "u32: length overflow" })
    }
}

pub fn elen_u16(n: usize) -> Result<U16W> {
    if n <= u16::max_value() as usize {
        Ok(U16W::from(n as u16))
    } else {
        Err(app_error!{ codec "u16: length overflow" })
    }
}

pub mod encoder {
    use super::*;

    pub fn varint_u32<TI>() -> PduEncoder<VarIntU32Encoder, TI> {
        PduEncoder::new(VarIntU32Encoder, elen_varint_u32)
    }

    pub fn fixed_u32<TI>() -> PduPairEncoder<U32W, TI> {
        PduPairEncoder::<U32W, TI> {
            h: PhantomData,
            t: PhantomData,
            f: elen_u32
        }
    }
}

pub mod decoder {
    use super::*;
    pub fn varint_u32<TI: PduDes>() -> PduDecoder<VarIntU32Decoder, TI> {
        pdu_decoder(
            VarIntU32Decoder::new(),
            |sz| sz as usize
        )
    }
    pub fn fixed_u32<TI: PduDes + Debug>() -> PduPairDecoder<U32W, TI> {
        pdu_pair_decoder::<U32W, TI>()
    }
}


#[derive(Debug)]
pub struct U32W {
    v: u32
}

impl FixedSizePdu for U32W {
    const PDU_SIZE: usize = U32_BYTES;
}

impl From<u32> for U32W {
    fn from(v: u32) -> Self { U32W { v } }
}

impl Into<u32> for U32W {
    fn into(self) -> u32 { self.v }
}

impl Into<usize> for U32W {
    fn into(self) -> usize { self.v as usize }
}

impl PduDes for U32W {
    fn from_bytes(b: BytesMut) -> Result<Self> {
        if b.len() != 4 {
            Err(Error::Other(Cow::from(format!("<U32W as PduDes>::from_bytes: invalid length ({})", b.len()))))
        } else {
            Ok(BigEndian::read_u32(&b).into())
        }
    }
}

impl PduSer for U32W {
    fn serialized_len(&mut self) -> usize { U32_BYTES }
    fn encode(self, b: &mut BytesMut) -> Result<()> {
        b.reserve(4);
        b.put_u32::<BigEndian>(self.v);
        Ok(())
    }
}

#[derive(Debug)]
pub struct U16W {
    v: u16
}

impl FixedSizePdu for U16W {
    const PDU_SIZE: usize = U16_BYTES;
}
impl From<u16> for U16W {
    fn from(v: u16) -> Self { U16W { v } }
}

impl Into<u16> for U16W {
    fn into(self) -> u16 { self.v }
}

impl Into<usize> for U16W {
    fn into(self) -> usize { self.v as usize }
}

impl PduDes for U16W {
    fn from_bytes(b: BytesMut) -> Result<Self> {
        if b.len() != U16_BYTES {
            Err(Error::Other(Cow::from(format!("<U16W as PduDes>::from_bytes: invalid length ({})", b.len()))))
        } else {
            Ok(BigEndian::read_u16(&b).into())
        }
    }
}

impl PduSer for U16W {
    fn serialized_len(&mut self) -> usize { U16_BYTES }
    fn encode(self, b: &mut BytesMut) -> Result<()> {
        b.reserve(U16_BYTES);
        b.put_u16::<BigEndian>(self.v);
        Ok(())
    }
}



#[test]
fn test_var_int_u32_encoder() {
    macro_rules! ck {
        { $v:expr, $a:expr } => {
            let mut w = BytesMut::with_capacity(5);
            let r = VarIntU32Encoder.encode($v, &mut w);
            assert!(r.is_ok());
            assert_eq!(&w[..], $a);
        }
    }

    ck!(300, [0b1010_1100, 0b0000_0010]);
    ck!(0x00000000, [0x00]);
    ck!(0x00000040, [0x40]);
    ck!(0x0000007F, [0x7F]);
}

#[test]
fn test_var_int_u32_decoder() {
    macro_rules! ck {
        { $v:expr, $a:expr } => {
            let mut w = BytesMut::with_capacity(5);
            w.put_slice(&$a);
            let mut d = VarIntU32Decoder::new();
            let r = d.decode(&mut w);
            assert_eq!(Some(Some($v)), r.ok());
            assert!(w.is_empty());
        }
    }

    ck!(300, [0b1010_1100, 0b0000_0010]);
    ck!(0x00000000, [0x00]);
    ck!(0x00000040, [0x40]);
    ck!(0x0000007F, [0x7F]);
}



#[test]
fn test_pdu_decoder_u16() {
    let mut data = BytesMut::with_capacity(256);
    data.put_slice(&[0, 3, 65, 66, 67, 68]);
    let mut rdr =
        pdu_pair_decoder::<U16W, BytesMutW>();
    assert_eq!(rdr.decode(&mut data).ok(), Some(Some(BytesMutW::from_static(&[65, 66, 67]))));
    assert_eq!(&data as &[u8], &[68]);

    let mut data = BytesMut::with_capacity(256);
    let mut rdr =
        pdu_pair_decoder::<U16W, BytesMutW>();;
    assert_eq!(rdr.decode(&mut data).ok(), Some(None));
    data.put_u8(0);
    assert_eq!(rdr.decode(&mut data).ok(), Some(None));
    data.put_u8(3);
    assert_eq!(rdr.decode(&mut data).ok(), Some(None));
    data.put_u8(65);
    assert_eq!(rdr.decode(&mut data).ok(), Some(None));
    data.put_u8(66);
    assert_eq!(rdr.decode(&mut data).ok(), Some(None));
    data.put_u8(67);
    assert_eq!(rdr.decode(&mut data).ok(), Some(Some(BytesMutW::from_static(&[65, 66, 67]))));
    assert_eq!(&data as &[u8], &[] as &[u8]);


    let mut data = BytesMut::with_capacity(261);
    data.put_slice(&[1, 1]);
    data.put_slice(&[0; 259]);
    let mut rdr =
        pdu_pair_decoder::<U16W, BytesMutW>();
    let r = rdr.decode(&mut data);
    match r {
        Ok(Some(pdu)) => assert_eq!(pdu.b.len(), 257),
        _ => panic!("assertion")
    }
    assert_eq!(data.len(), 2);
}

#[test]
fn test_pdu_reader_u32() {
    let mut data = BytesMut::with_capacity(256);
    data.put_slice(&[0, 0, 0, 3, 65, 66, 67, 68]);
    let mut rdr =
        pdu_pair_decoder::<U32W, BytesMutW>();
    assert_eq!(rdr.decode(&mut data).ok(), Some(Some(BytesMutW::from_static(&[65, 66, 67]))));
    assert_eq!(&data as &[u8], &[68]);

    let mut data = BytesMut::with_capacity(256);
    let mut rdr =
        pdu_pair_decoder::<U32W, BytesMutW>();
    assert_eq!(rdr.decode(&mut data).ok(), Some(None));
    data.put_u8(0);
    assert_eq!(rdr.decode(&mut data).ok(), Some(None));
    data.put_u8(0);
    assert_eq!(rdr.decode(&mut data).ok(), Some(None));
    data.put_u8(0);
    assert_eq!(rdr.decode(&mut data).ok(), Some(None));
    data.put_u8(3);
    assert_eq!(rdr.decode(&mut data).ok(), Some(None));
    data.put_u8(65);
    assert_eq!(rdr.decode(&mut data).ok(), Some(None));
    data.put_u8(66);
    assert_eq!(rdr.decode(&mut data).ok(), Some(None));
    data.put_u8(67);
    assert_eq!(rdr.decode(&mut data).ok(), Some(Some(BytesMutW::from_static(&[65, 66, 67]))));
    assert_eq!(&data as &[u8], &[] as &[u8]);


    let mut data = BytesMut::with_capacity(263);
    data.put_slice(&[0, 0, 1, 1]);
    data.put_slice(&[0; 259]);
    let mut rdr =
        pdu_pair_decoder::<U32W, BytesMutW>();
    let r = rdr.decode(&mut data);
    match r {
        Ok(Some(pdu)) => assert_eq!(pdu.b.len(), 257),
        _ => panic!("assertion")
    }
    assert_eq!(data.len(), 2);
}

#[test]
fn test_pdu_reader_u32_2() {
    let mut rdr =
        pdu_pair_decoder::<U32W, BytesMutW>();

    let mut data = BytesMut::with_capacity(256);
    data.put_slice(&[0, 0, 0, 3, 65, 66]);
    let r = rdr.decode(&mut data).ok();
    assert_eq!(r, Some(None));

    data.put(67 as u8);
    let r = rdr.decode(&mut data).ok();
    assert_eq!(r, Some(Some(BytesMutW::from_static(&[65, 66, 67]))));
}

#[test]
fn test_pdu_reader_var_int() {
    let mut data = BytesMut::with_capacity(256);
    data.put_slice(&[3, 65, 66, 67, 68]);
    let mut rdr =
        pdu_decoder::<VarIntU32Decoder, BytesMutW>(VarIntU32Decoder::new(), |w| w as usize);
    assert_eq!(rdr.decode(&mut data).ok(), Some(Some(BytesMutW::from_static(&[65, 66, 67]))));
    assert_eq!(&data as &[u8], &[68]);

    let mut data = BytesMut::with_capacity(256);
    let mut rdr =
        pdu_decoder::<VarIntU32Decoder, BytesMutW>(VarIntU32Decoder::new(), |w| w as usize);
    assert_eq!(rdr.decode(&mut data).ok(), Some(None));
    data.put_u8(3);
    assert_eq!(rdr.decode(&mut data).ok(), Some(None));
    data.put_u8(65);
    assert_eq!(rdr.decode(&mut data).ok(), Some(None));
    data.put_u8(66);
    assert_eq!(rdr.decode(&mut data).ok(), Some(None));
    data.put_u8(67);
    assert_eq!(rdr.decode(&mut data).ok(), Some(Some(BytesMutW::from_static(&[65, 66, 67]))));
    assert_eq!(&data as &[u8], &[] as &[u8]);


    let mut data = BytesMut::with_capacity(261);
    data.put_slice(&[0x81, 0x02]);
    data.put_slice(&[0; 259]);
    let mut rdr =
        pdu_decoder::<VarIntU32Decoder, BytesMutW>(VarIntU32Decoder::new(), |w| w as usize);
    let r = rdr.decode(&mut data);
    match r {
        Ok(Some(pdu)) => assert_eq!(pdu.b.len(), 257),
        _ => panic!("assertion")
    }
    assert_eq!(data.len(), 2);
}



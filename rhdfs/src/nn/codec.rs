///! Namenode protocol codec (Standard port 8020)

use std::fmt::Debug;
use tokio_io::codec::{Decoder, Encoder};
use bytes::{BytesMut, BufMut};
use byteorder::BigEndian;
use codec_tools::*;
use util::*;
use protobuf_api::*;
use result::ErrorConverter;
use *;


const RPC_VERSION: u8   = 0x09;
const SERVICE_CLASS: u8 = 0x00;
const AUTH_PROTOCOL: u8 = 0x00;
const PROTOCOL_CLASS: &str = "org.apache.hadoop.hdfs.protocol.ClientProtocol";
const PROTOCOL_CLASS_VERSION: u64  = 1;
const HANDSHAKE_CALL_ID: i32 = -3;
//standbyExceptionClass = "org.apache.hadoop.ipc.StandbyException"

#[derive(Debug)]
pub struct HandshakeContext {
    client_id: Vec<u8>,
    effective_user: String
}

impl HandshakeContext {
    pub fn new(client_id: Vec<u8>, effective_user: String) -> HandshakeContext {
        HandshakeContext { client_id, effective_user }
    }
}

/// A handshake packet:
///```text
/// +-----------------------------------------------------------+
/// |  Header, 4 bytes ("hrpc")                                 |
/// +-----------------------------------------------------------+
/// |  Version, 1 byte (default verion 0x09)                    |
/// +-----------------------------------------------------------+
/// |  RPC service class, 1 byte (0x00)                         |
/// +-----------------------------------------------------------+
/// |  Auth protocol, 1 byte (Auth method None = 0x00)          |
/// +-----------------------------------------------------------+
/// |  uint32 length of the next two parts                      |
/// +-----------------------------------------------------------+
/// |  varint length + RpcRequestHeaderProto                    |
/// +-----------------------------------------------------------+
/// |  varint length + IpcConnectionContextProto                |
/// +-----------------------------------------------------------+
/// ```
fn handshake_packet(hcx: HandshakeContext, dst: &mut BytesMut) -> Result<()> {
    dst.put_slice(&[
        0x68, 0x72, 0x70, 0x63, // "hrpc"
        RPC_VERSION, SERVICE_CLASS, AUTH_PROTOCOL
    ]);

    let mut tail = dst.split_off(7);

    let body = BiPdu::new(
        pb_cons!(RpcRequestHeaderProto,
            rpc_kind:  RpcKindProto::RPC_PROTOCOL_BUFFER,
            rpc_op: RpcRequestHeaderProto_OperationProto::RPC_FINAL_PACKET,
            call_id: HANDSHAKE_CALL_ID,
            client_id: hcx.client_id
        ),
        Some(pb_cons!(IpcConnectionContextProto,
            user_info: pb_cons!(UserInformationProto, effective_user: hcx.effective_user)
         ))
    );
    let mut body_enc = encoder::fixed_u32();

    let rv = body_enc.encode(body, &mut tail);
    dst.unsplit(tail);
    rv
}

#[derive(Debug)]
pub struct OpContext {
    pub client_id: Vec<u8>,
    pub call_id: i32,
    pub method_name: String
}

/// Build a request packet.
/// The namenode request has the following framing:
/// ```text
/// +----------------------------------------------------------------+
/// |  uint32 length of the next three parts                         |
/// +----------------------------------------------------------------+
/// |  varint length + RpcRequestHeaderProto                         |
/// +----------------------------------------------------------------+
/// |  varint length + RequestHeaderProto                            |
/// +----------------------------------------------------------------+
/// |  varint length + Request                                       |
/// +----------------------------------------------------------------+
/// ```

fn request_packet<M: PduSer>(ocx: OpContext, r: M, dst: &mut BytesMut) -> Result<()> {
    let body = TriPdu::new(
        pb_cons!(RpcRequestHeaderProto,
            rpc_kind:  RpcKindProto::RPC_PROTOCOL_BUFFER,
            rpc_op: RpcRequestHeaderProto_OperationProto::RPC_FINAL_PACKET,
            call_id: ocx.call_id,
            client_id: ocx.client_id
        ),
        pb_cons!(RequestHeaderProto,
            method_name: ocx.method_name,
            declaring_class_protocol_name: PROTOCOL_CLASS.to_owned(),
            client_protocol_version: PROTOCOL_CLASS_VERSION
        ),
        r
    );
    let mut body_enc = encoder::fixed_u32();

    body_enc.encode(body, dst)
}


#[derive(Debug)]
pub enum RpcReq<Q: Debug> {
    Handshake(HandshakeContext),
    Operation(OpContext, Q)
}

#[derive(Debug)]
pub struct RpcRsp<R: Debug> {
    pub header: RpcResponseHeaderProto,
    pub payload: Option<R>
}

pub trait RpcCodecA: Debug {
    type Q: PduSer + Debug;
    type R: Debug;
    type S; //: Decoder<Item=RpcRsp<A::R>, Error=Error>

    /// Given an incoming message, produce a pair of serializable PDU, ready for encoding, and a
    /// decoder
    fn state(req: &Self::Q) -> Self::S;
}

#[derive(Debug)]
pub enum RpcCodec<A> where A: RpcCodecA, A::S: Decoder<Item=RpcRsp<A::R>, Error=Error> + Debug {
    Uninitialized,
    Null,
    Request(A::S)
}

impl<A> RpcCodec<A> where A: RpcCodecA, A::S: Decoder<Item=RpcRsp<A::R>, Error=Error> + Debug {
    pub fn new() -> RpcCodec<A> { RpcCodec::Uninitialized }
    fn null() -> RpcCodec<A> { RpcCodec::Null }
}

#[inline]
fn dresult_c<R1, R2, F, A>(r: Result<Option<R1>>, conv: F) -> SnV<RpcCodec<A>, Result<Option<R2>>>
    where
        F: FnOnce(R1) -> R2,
        A: RpcCodecA,
        A::S: Decoder<Item=RpcRsp<A::R>, Error=Error> + Debug
{
    match r {
        Ok(Some(r1)) => SnV::SV(RpcCodec::Null, Ok(Some(conv(r1)))),
        Ok(None) => SnV::V(Ok(None)),
        Err(e) => SnV::V(Err(e))
    }
}

impl<A> Encoder for RpcCodec<A> where
    A: RpcCodecA,
    A::S: Decoder<Item=RpcRsp<A::R>, Error=Error> + Debug
{
    type Item = RpcReq<A::Q>;
    type Error = IoError;

    fn encode(&mut self, item: Self::Item, dst: &mut BytesMut) -> IoResult<()> {
        logging_switch_state_f("NnCodec::encode", self, |s| match s {
            &mut RpcCodec::Uninitialized =>
                SnV::SV(RpcCodec::Null,
                        if let RpcReq::Handshake(hcx) = item {
                            handshake_packet(hcx, dst)
                        } else {
                            Err(app_error!(codec "encoder: invalid Uninitialized/{:?} (handshake expected)", item))
                        }),
            &mut RpcCodec::Null =>
            //Do the work
                match item {
                    RpcReq::Operation(ocx, mut payload) => {
                        let s= A::state(&mut payload);
                        SnV::SV(RpcCodec::Request(s), request_packet(ocx, payload, dst))
                    },
                    e =>
                        SnV::V(Err(app_error!(codec "encoder: invalid Null/{:?}", e)))
                }
            &mut ref s =>
                SnV::SV(RpcCodec::Null, Err(app_error!(codec "encoder: invalid {:?}/{:?}", s, item)))
        }).c_err()
    }
}


impl<A> Decoder for RpcCodec<A> where
    A: RpcCodecA,
    A::S: Decoder<Item=RpcRsp<A::R>, Error=Error> + Debug,
    A::R: Debug
{
    type Item = RpcRsp<A::R>;
    type Error = IoError;
    fn decode(&mut self, src: &mut BytesMut) -> IoResult<Option<Self::Item>> {
        logging_switch_state_f("RpcCodec::decode", self, |s| match s {
            &mut RpcCodec::Request(ref mut s) =>
                dresult_c(s.decode(src), |x| x),
            &mut ref s =>
                SnV::SV(RpcCodec::Null, Err(app_error!(codec "decoder: invalid state {:?}", s)))
        }).c_err()
    }
}


/// Parses a response from the namenode.
/// ```text
/// +-----------------------------------------------------------+
/// |  uint32 length of the next two parts                      |
/// +-----------------------------------------------------------+
/// |  varint length + RpcResponseHeaderProto                   |
/// +-----------------------------------------------------------+
/// |  varint length + Response                                 |
/// +-----------------------------------------------------------+
/// ```
#[derive(Debug)]
pub struct RpcDecoder<MR, R> {
    internal: PduPairDecoder<U32W, BiPdu<RpcResponseHeaderProto, MR>>,
    conv: fn(MR) -> R
}

impl<MR, R> RpcDecoder<MR, R> where MR: PduDes + Debug {
    pub fn new(conv: fn(MR) -> R) -> RpcDecoder<MR, R> {
        RpcDecoder { internal: decoder::fixed_u32(), conv }
    }
}

impl<MR, R> Decoder for RpcDecoder<MR, R> where MR: PduDes + Debug, R: Debug {
    type Item = RpcRsp<R>;
    type Error = Error;

    fn decode(&mut self, src: &mut BytesMut) -> Result<Option<Self::Item>> {
        match self.internal.decode(src) {
            Ok(Some(BiPdu { a, b })) => Ok(Some(RpcRsp { header: a, payload: b.map(|w|(self.conv)(w)) })),
            Ok(None) => Ok(None),
            Err(e) => Err(e)
        }
    }
}

//--------------------------------------------------------------------------------------------------

#[derive(Debug)]
pub enum NnQ {
    GetListing(GetListingRequestProto)
}

impl PduSer for NnQ {
    fn serialized_len(&mut self) -> usize {
        match self {
            &mut NnQ::GetListing(ref mut glrp) => glrp.serialized_len()
        }
    }

    fn encode(self, b: &mut BytesMut) -> Result<()> {
        match self {
            NnQ::GetListing(glrp) => glrp.encode(b)
        }
    }
}

#[derive(Debug)]
pub enum NnR {
    GetListing(GetListingResponseProto)
}

#[derive(Debug)]
pub enum NnS {
    GetListing(RpcDecoder<GetListingResponseProto, NnR>)
}

impl Decoder for NnS {
    type Item = RpcRsp<NnR>;
    type Error = Error;

    fn decode(&mut self, src: &mut BytesMut) -> Result<Option<Self::Item>> {
        match self {
            &mut NnS::GetListing(ref mut p) => p.decode(src)
        }
    }
}


#[derive(Debug)]
pub struct NnCodecA;
impl RpcCodecA for NnCodecA {
    type Q = NnQ;
    type R = NnR;
    type S = NnS;

    fn state<'a>(req: &NnQ) -> NnS {
        match req {
            &NnQ::GetListing(..) => NnS::GetListing(RpcDecoder::new(|o| NnR::GetListing(o)))
        }
    }
}

pub type NnCodec = RpcCodec<NnCodecA>;


#[test]
fn test_g_nn_codec_handshake() {
    use util::test::*;
    let mut c = RpcCodec::<NnCodecA>::new();
    let mut b = BytesMut::with_capacity(1024);//new();

    let client_id = vec![1, 2, 3, 4, 4, 3, 2, 1];
    let user = "cloudera".to_owned();

    let hcx = HandshakeContext {
        client_id: client_id.clone(),
        effective_user: user.clone()
    };

    let r = c.encode(RpcReq::Handshake(hcx), &mut b);
    //println!("{:?}", r);
    assert!(r.is_ok());
    assert_eq!(
        &b, &"68:72:70:63:09:00:00:00:00:00:1e:10:08:02:10:00:18:05:22:08:01:02:03:04:04:03:02:01:\
    0c:12:0a:0a:08:63:6c:6f:75:64:65:72:61".to_bytes());
    assert_enum_variant!(c, RpcCodec::Null);
}

#[test]
fn test_g_nn_codec_read_listing() {
    use util::test::*;
    extern crate env_logger;
    env_logger::init();

    let mut c = RpcCodec::<NnCodecA>::null();
    let mut b =  BytesMut::with_capacity(1024);//new();

    let client_id = vec![1, 2, 3, 4, 4, 3, 2, 1];

    let ocx = OpContext {
        client_id: client_id.clone(),
        call_id: 0,
        method_name: "getListing".to_owned()
    };

    let gl = pb_cons!(GetListingRequestProto,
        src: "/".to_owned(),
        start_after: vec![],
        need_location: false
        );

    let r = c.encode(RpcReq::Operation(ocx, NnQ::GetListing(gl)), &mut b);
    //println!("{:?}", r);
    assert!(r.is_ok());
    //println!("{}", CBinary(&b));
    assert_eq!(
        &b, &"00:00:00:58:10:08:02:10:00:18:00:22:08:01:02:03:04:04:\
    03:02:01:3e:0a:0a:67:65:74:4c:69:73:74:69:6e:67:12:2e:6f:72:67:2e:61:70:61:63:68:65:2e:68:61:\
    64:6f:6f:70:2e:68:64:66:73:2e:70:72:6f:74:6f:63:6f:6c:2e:43:6c:69:65:6e:74:50:72:6f:74:6f:63:\
    6f:6c:18:01:07:0a:01:2f:12:00:18:00".to_bytes());
    assert_enum_variant!(&c, &RpcCodec::Request(NnS::GetListing(..)));

    //test partial pdu decoding
    let mut br = BytesMut::with_capacity(1024);
    br.put_slice(&"\
    00:00:01:70:12:08:00:10:00:18:09:3a:08:01:02:03:04:04:03:02:01:40:01:db:02:0a:d8:02:0a:\
    3d:08:01:12:0a:62:65:6e:63:68:6d:61:72:6b:73:18:00:22:03:08:ff:03:2a:04:68:64:66:73:32:0a:73:\
    75:70:65:72:67:72:6f:75:70:38:e1:d7:df:d6:d5:2b:40:00:50:00:58:00:68:8e:80:01:70:00:80:01:00"
        .to_bytes());

    let r = c.decode(&mut br);
    assert_enum_variant!(r, Ok(None));
    //println!("<1> {:?}", c);

    br.put_slice(&"\
    0a:39:08:01:12:05:68:62:61:73:65:18:00:22:03:08:ed:03:2a:05:68:62:61:73:65:32:0a:73:75:70:65:\
    72:67:72:6f:75:70:38:b4:be:e4:95:f7:2b:40:00:50:00:58:00:68:8d:80:01:70:09:80:01:00:0a:31:08:\
    01:12:04:73:6f:6c:72:18:00:22:03:08:ed:03:2a:04:73:6f:6c:72:32:04:73:6f:6c:72:38:e1:91:e8:d6:\
    d5:2b:40:00:50:00:58:00:68:f9:81:01:70:00:80:01:00:0a:36:08:01:12:03:74:6d:70:18:00:22:03:08:\
    ff:07:2a:04:68:64:66:73:32:0a:73:75:70:65:72:67:72:6f:75:70:38:eb:b2:ab:b4:97:2c:40:00:50:00:\
    58:00:68:84:80:01:70:05:80:01:00:0a:37:08:01:12:04:75:73:65:72:18:00:22:03:08:ed:03:2a:04:68:\
    64:66:73:32:0a:73:75:70:65:72:67:72:6f:75:70:38:b7:b5:e6:d6:d5:2b:40:00:50:00:58:00:68:82:80:\
    01:70:08:80:01:00:0a:36:08:01:12:03:76:61:72:18:00:22:03:08:ed:03:2a:04:68:64:66:73:32:0a:73:\
    75:70:65:72:67:72:6f:75:70:38:a4:f2:e5:d6:d5:2b:40:00:50:00:58:00:68:85:80:01:70:02:80:01:00:\
    10:00".to_bytes());

    let r = c.decode(&mut br);
    assert_enum_variant!(c, RpcCodec::Null);
    //println!("{:?}", r);
    let (header, resp) = match r {
        Ok(Some(RpcRsp { header, payload: Some(NnR::GetListing(gl)) })) => (header, gl),
        other => panic!("Invalid resp: {:?}", other)
    };

    let (r_call_id, r_status, r_client_id) =
        pb_decons!(RpcResponseHeaderProto, header, call_id, status, client_id);

    assert_eq!(r_call_id, 0);
    assert_eq!(r_client_id as &[u8], &client_id as &[u8]);
    assert_eq!(r_status, RpcResponseHeaderProto_RpcStatusProto::SUCCESS);


    /*

    let mut br = BytesMut::from(
        "00:00:01:70:12:08:00:10:00:18:09:3a:08:01:02:03:04:04:03:02:01:40:01:db:02:0a:d8:02:0a:\
    3d:08:01:12:0a:62:65:6e:63:68:6d:61:72:6b:73:18:00:22:03:08:ff:03:2a:04:68:64:66:73:32:0a:73:\
    75:70:65:72:67:72:6f:75:70:38:e1:d7:df:d6:d5:2b:40:00:50:00:58:00:68:8e:80:01:70:00:80:01:00:\
    0a:39:08:01:12:05:68:62:61:73:65:18:00:22:03:08:ed:03:2a:05:68:62:61:73:65:32:0a:73:75:70:65:\
    72:67:72:6f:75:70:38:b4:be:e4:95:f7:2b:40:00:50:00:58:00:68:8d:80:01:70:09:80:01:00:0a:31:08:\
    01:12:04:73:6f:6c:72:18:00:22:03:08:ed:03:2a:04:73:6f:6c:72:32:04:73:6f:6c:72:38:e1:91:e8:d6:\
    d5:2b:40:00:50:00:58:00:68:f9:81:01:70:00:80:01:00:0a:36:08:01:12:03:74:6d:70:18:00:22:03:08:\
    ff:07:2a:04:68:64:66:73:32:0a:73:75:70:65:72:67:72:6f:75:70:38:eb:b2:ab:b4:97:2c:40:00:50:00:\
    58:00:68:84:80:01:70:05:80:01:00:0a:37:08:01:12:04:75:73:65:72:18:00:22:03:08:ed:03:2a:04:68:\
    64:66:73:32:0a:73:75:70:65:72:67:72:6f:75:70:38:b7:b5:e6:d6:d5:2b:40:00:50:00:58:00:68:82:80:\
    01:70:08:80:01:00:0a:36:08:01:12:03:76:61:72:18:00:22:03:08:ed:03:2a:04:68:64:66:73:32:0a:73:\
    75:70:65:72:67:72:6f:75:70:38:a4:f2:e5:d6:d5:2b:40:00:50:00:58:00:68:85:80:01:70:02:80:01:00:\
    10:00".to_bytes())
    */


}

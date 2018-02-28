use protocolpb::proto::hadoop::RpcHeader::{RpcRequestHeaderProto, RpcResponseHeaderProto,
                                   RpcResponseHeaderProto_RpcStatusProto,
                                   //RpcResponseHeaderProto_RpcErrorCodeProto,
                                   RpcKindProto, RpcRequestHeaderProto_OperationProto};
use protocolpb::proto::hadoop::IpcConnectionContext::{IpcConnectionContextProto, UserInformationProto};
use protocolpb::proto::hadoop::ProtobufRpcEngine::RequestHeaderProto;
use protobuf::{CodedOutputStream, CodedInputStream, Message, ProtobufResult};
use byteorder::{BigEndian, WriteBytesExt};
use ::Result;
use util;

use std::io::Write;

const RPC_VERSION: u8   = 0x09;
const SERVICE_CLASS: u8 = 0x00;
const AUTH_PROTOCOL: u8 = 0x00;
const PROTOCOL_CLASS: &str = "org.apache.hadoop.hdfs.protocol.ClientProtocol";
const PROTOCOL_CLASS_VERSION: u64  = 1;
const HANDSHAKE_CALL_ID: i32 = -3;
//standbyExceptionClass = "org.apache.hadoop.ipc.StandbyException"


fn append_rpc_packet_2<M1: Message, M2: Message>(b: &mut Vec<u8>, m1: M1, m2: M2) -> ProtobufResult<()> {
    let mut c = Vec::new();
    {
        let mut os = CodedOutputStream::new(&mut c);
        Ok(())
            .and_then(|()| os.write_raw_varint32(m1.compute_size()))
            .and_then(|()| m1.write_to_with_cached_sizes(&mut os))
            .and_then(|()| os.write_raw_varint32(m2.compute_size()))
            .and_then(|()| m2.write_to_with_cached_sizes(&mut os))
            .and_then(|()| os.flush())
    }?;
    b.write_u32::<BigEndian>(c.len() as u32)?;
    b.write(&c)?;
    b.flush()?;
    Ok(())
}

fn append_rpc_packet_3<M1: Message, M2: Message, M3: Message>(b: &mut Vec<u8>, m1: M1, m2: M2, m3: M3) -> ProtobufResult<()> {
    let mut os = CodedOutputStream::new(b);
    Ok(())
        .and_then(|()| os.write_raw_varint32(m1.compute_size()))
        .and_then(|()| m1.write_to_with_cached_sizes(&mut os))
        .and_then(|()| os.write_raw_varint32(m2.compute_size()))
        .and_then(|()| m2.write_to_with_cached_sizes(&mut os))
        .and_then(|()| os.write_raw_varint32(m3.compute_size()))
        .and_then(|()| m3.write_to_with_cached_sizes(&mut os))
        .and_then(|()| os.flush())
}


// A handshake packet:
// +-----------------------------------------------------------+
// |  Header, 4 bytes ("hrpc")                                 |
// +-----------------------------------------------------------+
// |  Version, 1 byte (default verion 0x09)                    |
// +-----------------------------------------------------------+
// |  RPC service class, 1 byte (0x00)                         |
// +-----------------------------------------------------------+
// |  Auth protocol, 1 byte (Auth method None = 0x00)          |
// +-----------------------------------------------------------+
// |  uint32 length of the next two parts                      |
// +-----------------------------------------------------------+
// |  varint length + RpcRequestHeaderProto                    |
// +-----------------------------------------------------------+
// |  varint length + IpcConnectionContextProto                |
// +-----------------------------------------------------------+

pub fn handshake_packet(client_id: Vec<u8>, effective_user: String) -> ProtobufResult<Vec<u8>> {
    let mut rrh = RpcRequestHeaderProto::new();
    rrh.set_rpcKind(RpcKindProto::RPC_PROTOCOL_BUFFER);
    rrh.set_rpcOp(RpcRequestHeaderProto_OperationProto::RPC_FINAL_PACKET);
    rrh.set_callId(HANDSHAKE_CALL_ID);
    rrh.set_clientId(client_id);

    let mut icc = IpcConnectionContextProto::new();
    icc.set_userInfo({
        let mut ui = UserInformationProto::new();
        ui.set_effectiveUser(effective_user);
        ui
    });

    let mut b = Vec::new();
    b.write(&[
        0x68, 0x72, 0x70, 0x63, // "hrpc"
        RPC_VERSION, SERVICE_CLASS, AUTH_PROTOCOL
    ])?;

    append_rpc_packet_2(&mut b, rrh, icc)?;
    Ok(b)
}

// RPC definitions

/// Build a request packet.
/// The namenode request has the following framing:
/// ```text
/// +----------------------------------------------------------------+
/// |  uint32 length of the next three parts (written in RPC framer) |
/// +----------------------------------------------------------------+
/// |  varint length + RpcRequestHeaderProto                         |
/// +----------------------------------------------------------------+
/// |  varint length + RequestHeaderProto                            |
/// +----------------------------------------------------------------+
/// |  varint length + Request                                       |
/// +----------------------------------------------------------------+
/// ```

pub fn request_packet<M: Message>(
    client_id: Vec<u8>, call_id: i32,
    method_name: String,
    r: M) -> ProtobufResult<Vec<u8>>
{
    let mut rrh = RpcRequestHeaderProto::new();
    rrh.set_rpcKind(RpcKindProto::RPC_PROTOCOL_BUFFER);
    rrh.set_rpcOp(RpcRequestHeaderProto_OperationProto::RPC_FINAL_PACKET);
    rrh.set_callId(call_id);
    rrh.set_clientId(client_id);

    let mut rh = RequestHeaderProto::new();
    rh.set_methodName(method_name);
    rh.set_declaringClassProtocolName(PROTOCOL_CLASS.to_owned());
    rh.set_clientProtocolVersion(PROTOCOL_CLASS_VERSION);

    let mut b = Vec::new();
    append_rpc_packet_3(&mut b, rrh, rh, r)?;
    Ok(b)
}

/// Parses a response from the namenode.
/// 'm' is an empty template M to be filled in and returned.
/// ```text
/// +-----------------------------------------------------------+
/// |  uint32 length of the next two parts (read in RPC framer) |
/// +-----------------------------------------------------------+
/// |  varint length + RpcResponseHeaderProto                   |
/// +-----------------------------------------------------------+
/// |  varint length + Response                                 |
/// +-----------------------------------------------------------+
/// ```

pub fn response_packet<M: Message>(d: Vec<u8>, mut m: M) -> Result<M> {
    //"length of the next two parts" has been read
    let mut i = CodedInputStream::from_bytes(&d);
    let sa = i.read_raw_varint64()? as usize;
    let mut b = util::vector_of_size(sa);
    i.read(&mut b)?;
    let mut rrh = RpcResponseHeaderProto::new();
    rrh.merge_from(&mut CodedInputStream::from_bytes(&b))?;

    #[inline]
    fn to_vector_of_size<T: Default + Clone>(v: &mut Vec<T>, n: usize) -> () {
        v.resize(n, T::default());
        v.shrink_to_fit();
    }

    //check for error
    if rrh.get_status() == RpcResponseHeaderProto_RpcStatusProto::SUCCESS {
        let sb = i.read_raw_varint64()? as usize;
        to_vector_of_size(&mut b, sb);
        i.read(&mut b)?;
        m.merge_from(&mut CodedInputStream::from_bytes(&b))?;
        Ok(m)
    } else {
        Err(::error::Error::RPC {
            protocol: "namenode".to_owned(),
            status: decode_status(rrh.get_status()),
            message: rrh.get_errorMsg().to_owned(),
            //TODO: make this up
            code: "?".to_owned(),
            exception_class: rrh.get_exceptionClassName().to_owned()
        })
    }
}

fn decode_status(st: RpcResponseHeaderProto_RpcStatusProto) -> String {
    match st {
        RpcResponseHeaderProto_RpcStatusProto::SUCCESS => "SUCCESS",
        RpcResponseHeaderProto_RpcStatusProto::ERROR => "ERROR",
        RpcResponseHeaderProto_RpcStatusProto::FATAL => "FATAL"
    }.to_owned()
}

use std::io::Write;
use protobuf::{CodedOutputStream, Message, MessageStatic};
use codec_tools::*;
use bytes::{BytesMut, BufMut};
use ::*;



impl<M> PduSer for M where M: Message {
    fn serialized_len(&mut self) -> usize {
        self.compute_size() as usize
    }
    fn encode(/*&*/self, b: &mut BytesMut) -> Result<()> {
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
    use protocolpb::proto::hdfs::datatransfer::*;
    use protocolpb::proto::hdfs::hdfs::*;

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

pub use protocolpb::proto::hdfs::datatransfer::{
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

pub use protocolpb::proto::hdfs::hdfs::{
    ExtendedBlockProto,
    ChecksumTypeProto,
    DirectoryListingProto,
    HdfsFileStatusProto,
    HdfsFileStatusProto_FileType
};

pub use protocolpb::proto::hdfs::ClientNamenodeProtocol::{
    GetListingRequestProto,
    GetListingResponseProto,
};

pub use protocolpb::proto::hadoop::RpcHeader::{
    RpcRequestHeaderProto,
    RpcResponseHeaderProto,
    RpcResponseHeaderProto_RpcStatusProto,
    RpcResponseHeaderProto_RpcErrorCodeProto,
    RpcKindProto,
    RpcRequestHeaderProto_OperationProto
};

pub use protocolpb::proto::hadoop::IpcConnectionContext::{
    IpcConnectionContextProto,
    UserInformationProto
};

pub use protocolpb::proto::hadoop::ProtobufRpcEngine::{
    RequestHeaderProto
};



/// Protobuf API abstraction layer.
/// Each PDU type field used must be declared here
macro_rules! pbdb {
//Namenode (client)
{ GetListingRequestProto, src, $a:tt } => { pbdbf!{ get_src, set_src, $a } };
{ GetListingRequestProto, start_after, $a:tt } => { pbdbf!{ get_startAfter, set_startAfter, $a } };
{ GetListingRequestProto, need_location, $a:tt } => { pbdbf!{ get_needLocation, set_needLocation, $a } };

{ GetListingResponseProto, dir_list, $a:tt } => { pbdbf!{ get_dirList, set_dirList, $a } };

{ DirectoryListingProto, partial_listing, $a:tt } => { pbdbf!{ get_partialListing, set_partialListing, $a } };
{ DirectoryListingProto, remaining_entries, $a:tt } => { pbdbf!{ get_remainingEntries, set_remainingEntries, $a } };

{ HdfsFileStatusProto, file_type, $a:tt }=> { pbdbf!{ get_fileType, set_fileType, $a } };
{ HdfsFileStatusProto, path, $a:tt }=> { pbdbf!{ get_path, set_path, $a } };
{ HdfsFileStatusProto, length, $a:tt }=> { pbdbf!{ get_length, set_length, $a } };
{ HdfsFileStatusProto, permission, $a:tt }=> { pbdbf!{ get_permission, set_permission, $a } };
{ HdfsFileStatusProto, owner, $a:tt }=> { pbdbf!{ get_owner, set_owner, $a } };
{ HdfsFileStatusProto, group, $a:tt }=> { pbdbf!{ get_group, set_group, $a } };
{ HdfsFileStatusProto, modification_time, $a:tt }=> { pbdbf!{ get_modification_time, set_modification_time, $a } };
{ HdfsFileStatusProto, access_time, $a:tt }=> { pbdbf!{ get_access_time, set_access_time, $a } };
{ HdfsFileStatusProto, symlink, $a:tt }=> { pbdbf!{ get_symlink, set_symlink, $a } };
{ HdfsFileStatusProto, block_replication, $a:tt }=> { pbdbf!{ get_block_replication, set_block_replication, $a } };
{ HdfsFileStatusProto, blocksize, $a:tt }=> { pbdbf!{ get_blocksize, set_blocksize, $a } };
{ HdfsFileStatusProto, locations, $a:tt }=> { pbdbf!{ get_locations, set_locations, $a } };
{ HdfsFileStatusProto, file_id, $a:tt }=> { pbdbf!{ get_fileId, set_fileId, $a } };
{ HdfsFileStatusProto, children_num, $a:tt }=> { pbdbf!{ get_childrenNum, set_childrenNum, $a } };
{ HdfsFileStatusProto, file_encryption_info, $a:tt }=> { pbdbf!{ get_fileEncryptionInfo, set_fileEncryptionInfo, $a } };
{ HdfsFileStatusProto, storage_policy, $a:tt }=> { pbdbf!{ get_storagePolicy, set_storagePolicy, $a } };

//RPC
{ RpcRequestHeaderProto, rpc_kind, $a:tt } => { pbdbf!{ get_rpcKind, set_rpcKind, $a } };
{ RpcRequestHeaderProto, rpc_op, $a:tt } => { pbdbf!{ get_rpcOp, set_rpcOp, $a } };
{ RpcRequestHeaderProto, call_id, $a:tt } => { pbdbf!{ get_callId, set_callId, $a } };
{ RpcRequestHeaderProto, client_id, $a:tt } => { pbdbf!{ get_clientId, set_clientId, $a } };

{ RpcResponseHeaderProto, call_id, $a:tt }=> { pbdbf!{ get_callId, set_callId, $a } };
{ RpcResponseHeaderProto, status, $a:tt }=> { pbdbf!{ get_status, set_status, $a } };
{ RpcResponseHeaderProto, server_ipc_version_num, $a:tt }=> { pbdbf!{ get_serverIpcVersionNum, set_serverIpcVersionNum, $a } };
{ RpcResponseHeaderProto, exception_class_name, $a:tt }=> { pbdbf!{ get_exceptionClassName, set_exceptionClassName, $a } };
{ RpcResponseHeaderProto, error_msg, $a:tt }=> { pbdbf!{ get_errorMsg, set_errorMsg, $a } };
{ RpcResponseHeaderProto, error_detail, $a:tt }=> { pbdbf!{ get_errorDetail, set_errorDetail, $a } };
{ RpcResponseHeaderProto, client_id, $a:tt }=> { pbdbf!{ get_clientId, set_clientId, $a } };
{ RpcResponseHeaderProto, retry_count, $a:tt }=> { pbdbf!{ get_retryCount, set_retryCount, $a } };

{ IpcConnectionContextProto, user_info, $a:tt } => { pbdbf!{ get_userInfo, set_userInfo, $a } };

{ UserInformationProto, effective_user, $a:tt } => { pbdbf!{ get_effectiveUser, set_effectiveUser, $a } };

{ RequestHeaderProto, method_name, $a:tt } => { pbdbf!{ get_methodName, set_methodName, $a } };
{ RequestHeaderProto, declaring_class_protocol_name, $a:tt } => { pbdbf!{ get_declaringClassProtocolName, set_declaringClassProtocolName, $a } };
{ RequestHeaderProto, client_protocol_version, $a:tt } => { pbdbf!{ get_clientProtocolVersion, set_clientProtocolVersion, $a } };

//Datatransfer
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
    use protocolpb::proto::hdfs::hdfs::ExtendedBlockProto;
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


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
    PacketHeaderProto,
    OpWriteBlockProto,
    OpWriteBlockProto_BlockConstructionStage,
    PipelineAckProto
};

pub use protocolpb::proto::hdfs::hdfs::{
    ExtendedBlockProto,
    ChecksumTypeProto,
    DirectoryListingProto,
    HdfsFileStatusProto,
    HdfsFileStatusProto_FileType,
    FsPermissionProto,
    LocatedBlocksProto,
    LocatedBlockProto,
    DatanodeInfoProto,
    DatanodeIDProto,
    StorageTypeProto
};

pub use protocolpb::proto::hdfs::ClientNamenodeProtocol::{
    GetListingRequestProto,
    GetListingResponseProto,
    GetBlockLocationsRequestProto,
    GetBlockLocationsResponseProto,
    CreateRequestProto,
    CreateResponseProto,
    AddBlockRequestProto,
    AddBlockResponseProto
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

pub use protocolpb::proto::hadoop::Security::{
    TokenProto
};


/// Protobuf API abstraction layer.
/// Each PDU type field used must be declared here
#[macro_export]
macro_rules! pbdb {
//ClientNamenodeProtocol.proto
{ GetListingRequestProto, src, $a:tt } => { pbdbf!{ get_src, set_src, $a } };
{ GetListingRequestProto, start_after, $a:tt } => { pbdbf!{ get_startAfter, set_startAfter, $a } };
{ GetListingRequestProto, need_location, $a:tt } => { pbdbf!{ get_needLocation, set_needLocation, $a } };

{ GetListingResponseProto, dir_list, $a:tt } => { pbdbf!{ get_dirList, set_dirList, take_dirList, $a } };

{ GetBlockLocationsRequestProto, src, $a:tt }=> { pbdbf!{ get_src, set_src, $a } };
{ GetBlockLocationsRequestProto, offset, $a:tt }=> { pbdbf!{ get_offset, set_offset, $a } };
{ GetBlockLocationsRequestProto, length, $a:tt }=> { pbdbf!{ get_length, set_length, $a } };

{ GetBlockLocationsResponseProto, locations, $a:tt }=> { pbdbf!{ take_locations, set_locations, $a } };

// (src, masked, client_name, create_flag, create_parent, replication, block_size, crypto_protocol_version)
{ CreateRequestProto, src, $a:tt }=> { pbdbf!{ get_src, set_src, $a } };
{ CreateRequestProto, masked, $a:tt }=> { pbdbf!{ get_masked, set_masked, $a } };
{ CreateRequestProto, client_name, $a:tt }=> { pbdbf!{ get_clientName, set_clientName, $a } };
{ CreateRequestProto, create_flag, $a:tt }=> { pbdbf!{ get_createFlag, set_createFlag, $a } };
{ CreateRequestProto, create_parent, $a:tt }=> { pbdbf!{ get_createParent, set_createParent, $a } };
{ CreateRequestProto, replication, $a:tt }=> { pbdbf!{ get_replication, set_replication, $a } };
{ CreateRequestProto, block_size, $a:tt }=> { pbdbf!{ get_blockSize, set_blockSize, $a } };
{ CreateRequestProto, crypto_protocol_version, $a:tt }=> { pbdbf!{ get_cryptoProtocolVersion, set_cryptoProtocolVersion, $a } };

// (fs)
{ CreateResponseProto, fs, $a:tt }=> { pbdbf!{ get_fs, set_fs, $a } };

// (src, client_name, previous, exclude_nodes, file_id, favored_nodes)
{ AddBlockRequestProto, src, $a:tt }=> { pbdbf!{ get_src, set_src, $a } };
{ AddBlockRequestProto, client_name, $a:tt }=> { pbdbf!{ get_clientName, set_clientName, $a } };
{ AddBlockRequestProto, previous, $a:tt }=> { pbdbf!{ get_previous, set_previous, $a } };
{ AddBlockRequestProto, exclude_nodes, $a:tt }=> { pbdbf!{ get_excludeNodes, set_excludeNodes, $a } };
{ AddBlockRequestProto, file_id, $a:tt }=> { pbdbf!{ get_fileId, set_fileId, $a } };
{ AddBlockRequestProto, favored_nodes, $a:tt }=> { pbdbf!{ get_favoredNodes, set_favoredNodes, $a } };

// (block)
{ AddBlockResponseProto, block, $a:tt }=> { pbdbf!{ get_block, set_block, $a } };

//----------------------------------------------------------------------------------------------------------------------
//hdfs.proto
{ DirectoryListingProto, partial_listing, $a:tt } => { pbdbf_rep!{ get_partialListing, set_partialListing, take_partialListing, $a } };
{ DirectoryListingProto, remaining_entries, $a:tt } => { pbdbf!{ get_remainingEntries, set_remainingEntries, $a } };

{ HdfsFileStatusProto, file_type, $a:tt }=> { pbdbf!{ get_fileType, set_fileType, $a } };
{ HdfsFileStatusProto, path, $a:tt }=> { pbdbf!{ get_path, set_path, take_path, $a } };
{ HdfsFileStatusProto, length, $a:tt }=> { pbdbf!{ get_length, set_length, $a } };
{ HdfsFileStatusProto, permission, $a:tt }=> { pbdbf!{ get_permission, set_permission, take_permission, $a } };
{ HdfsFileStatusProto, owner, $a:tt }=> { pbdbf!{ get_owner, set_owner, take_owner, $a } };
{ HdfsFileStatusProto, group, $a:tt }=> { pbdbf!{ get_group, set_group, take_group, $a } };
{ HdfsFileStatusProto, modification_time, $a:tt }=> { pbdbf!{ get_modification_time, set_modification_time, $a } };
{ HdfsFileStatusProto, access_time, $a:tt }=> { pbdbf!{ get_access_time, set_access_time, $a } };
{ HdfsFileStatusProto, symlink, $a:tt }=> { pbdbf!{ get_symlink, set_symlink, take_symlink, $a } };
{ HdfsFileStatusProto, block_replication, $a:tt }=> { pbdbf!{ get_block_replication, set_block_replication, $a } };
{ HdfsFileStatusProto, blocksize, $a:tt }=> { pbdbf!{ get_blocksize, set_blocksize, $a } };
{ HdfsFileStatusProto, locations, $a:tt }=> { pbdbf!{ get_locations, set_locations, take_locations, $a } };
{ HdfsFileStatusProto, file_id, $a:tt }=> { pbdbf!{ get_fileId, set_fileId, $a } };
{ HdfsFileStatusProto, children_num, $a:tt }=> { pbdbf!{ get_childrenNum, set_childrenNum, $a } };
{ HdfsFileStatusProto, file_encryption_info, $a:tt }=> { pbdbf!{ get_fileEncryptionInfo, set_fileEncryptionInfo, take_fileEncryptionInfo, $a } };
{ HdfsFileStatusProto, storage_policy, $a:tt }=> { pbdbf!{ get_storagePolicy, set_storagePolicy, $a } };

{ FsPermissionProto, perm, $a:tt }=> { pbdbf!{ get_perm, set_perm, $a } };

// (file_length, blocks, under_construction, last_block, is_last_block_complete, file_encryption_info)
{ LocatedBlocksProto, file_length, $a:tt }=> { pbdbf!{ get_fileLength, set_fileLength, $a } };
{ LocatedBlocksProto, blocks, $a:tt }=> { pbdbf_rep!{ get_blocks, set_blocks, take_blocks, $a } };
{ LocatedBlocksProto, under_construction, $a:tt }=> { pbdbf!{ get_underConstruction, set_underConstruction, $a } };
{ LocatedBlocksProto, last_block, $a:tt }=> { pbdbf!{ get_lastBlock, set_lastBlock, take_lastBlock, $a } };
{ LocatedBlocksProto, is_last_block_complete, $a:tt }=> { pbdbf!{ get_isLastBlockComplete, set_isLastBlockComplete, $a } };
{ LocatedBlocksProto, file_encryption_info, $a:tt }=> { pbdbf!{ get_fileEncryptionInfo, set_fileEncryptionInfo, take_fileEncryptionInfo, $a } };

// (b, offset, locs, corrupt, block_token, is_cached, storage_types, storage_ids)
{ LocatedBlockProto, b, $a:tt }=> { pbdbf!{ get_b, set_b, take_b, $a } };
{ LocatedBlockProto, offset, $a:tt }=> { pbdbf!{ get_offset, set_offset, $a } };
{ LocatedBlockProto, locs, $a:tt }=> { pbdbf_rep!{ get_locs, set_locs, take_locs, $a } };
{ LocatedBlockProto, corrupt, $a:tt }=> { pbdbf!{ get_corrupt, set_corrupt, $a } };
{ LocatedBlockProto, block_token, $a:tt }=> { pbdbf!{ get_blockToken, set_blockToken, take_blockToken, $a } };
{ LocatedBlockProto, is_cached, $a:tt }=> { pbdbf!{ get_isCached, set_isCached, take_isCached, $a } };
{ LocatedBlockProto, storage_types, $a:tt }=> { pbdbf!{ get_storageTypes, set_storageTypes, take_storageTypes, $a } };
{ LocatedBlockProto, storage_ids, $a:tt }=> { pbdbf!{ get_storageIDs, set_storageIDs, take_storageIDs, $a } };

{ DatanodeInfoProto, id, $a:tt }=> { pbdbf!{ get_id, set_id, take_id, $a } };
{ DatanodeInfoProto, capacity, $a:tt }=> { pbdbf!{ get_capacity, set_capacity, $a } };
{ DatanodeInfoProto, dfs_used, $a:tt }=> { pbdbf!{ get_dfsUsed, set_dfsUsed, $a } };
{ DatanodeInfoProto, remaining, $a:tt }=> { pbdbf!{ get_remaining, set_remaining, $a } };
{ DatanodeInfoProto, block_pool_used, $a:tt }=> { pbdbf!{ get_blockPoolUsed, set_blockPoolUsed, $a } };
{ DatanodeInfoProto, last_update, $a:tt }=> { pbdbf!{ get_lastUpdate, set_lastUpdate, $a } };
{ DatanodeInfoProto, xceiver_count, $a:tt }=> { pbdbf!{ get_xceiverCount, set_xceiverCount, $a } };
{ DatanodeInfoProto, location, $a:tt }=> { pbdbf!{ get_location, set_location, take_location, $a } };
{ DatanodeInfoProto, admin_state, $a:tt }=> { pbdbf!{ get_adminState, set_adminState, take_adminState, $a } };
{ DatanodeInfoProto, cache_capacity, $a:tt }=> { pbdbf!{ get_cacheCapacity, set_cacheCapacity, $a } };
{ DatanodeInfoProto, cache_used, $a:tt }=> { pbdbf!{ get_cacheUsed, set_cacheUsed, $a } };

{ DatanodeIDProto, ip_addr, $a:tt }=> { pbdbf!{ get_ipAddr, set_ipAddr, take_ipAddr, $a } };
{ DatanodeIDProto, host_name, $a:tt }=> { pbdbf!{ get_hostName, set_hostName, take_hostName, $a } };
{ DatanodeIDProto, datanode_uuid, $a:tt }=> { pbdbf!{ get_datanodeUuid, set_datanodeUuid, take_datanodeUuid, $a } };
{ DatanodeIDProto, xfer_port, $a:tt }=> { pbdbf!{ get_xferPort, set_xferPort, $a } };
{ DatanodeIDProto, info_port, $a:tt }=> { pbdbf!{ get_infoPort, set_infoPort, $a } };
{ DatanodeIDProto, ipc_port, $a:tt }=> { pbdbf!{ get_ipcPort, set_ipcPort, $a } };
{ DatanodeIDProto, info_secure_port, $a:tt }=> { pbdbf!{ get_infoSecurePort, set_infoSecurePort, $a } };

//RPC
{ RpcRequestHeaderProto, rpc_kind, $a:tt } => { pbdbf!{ get_rpcKind, set_rpcKind, $a } };
{ RpcRequestHeaderProto, rpc_op, $a:tt } => { pbdbf!{ get_rpcOp, set_rpcOp, $a } };
{ RpcRequestHeaderProto, call_id, $a:tt } => { pbdbf!{ get_callId, set_callId, $a } };
{ RpcRequestHeaderProto, client_id, $a:tt } => { pbdbf!{ get_clientId, set_clientId, $a } };

{ RpcResponseHeaderProto, call_id, $a:tt }=> { pbdbf!{ get_callId, set_callId, $a } };
{ RpcResponseHeaderProto, status, $a:tt }=> { pbdbf!{ get_status, set_status, $a } };
{ RpcResponseHeaderProto, server_ipc_version_num, $a:tt }=> { pbdbf!{ get_serverIpcVersionNum, set_serverIpcVersionNum, $a } };
{ RpcResponseHeaderProto, exception_class_name, $a:tt }=> { pbdbf!{ get_exceptionClassName, set_exceptionClassName, take_exceptionClassName, $a } };
{ RpcResponseHeaderProto, error_msg, $a:tt }=> { pbdbf!{ get_errorMsg, set_errorMsg, take_errorMsg, $a } };
{ RpcResponseHeaderProto, error_detail, $a:tt }=> { pbdbf!{ get_errorDetail, set_errorDetail, $a } };
{ RpcResponseHeaderProto, client_id, $a:tt }=> { pbdbf!{ get_clientId, set_clientId, take_clientId, $a } };
{ RpcResponseHeaderProto, retry_count, $a:tt }=> { pbdbf!{ get_retryCount, set_retryCount, $a } };

{ IpcConnectionContextProto, user_info, $a:tt } => { pbdbf!{ get_userInfo, set_userInfo, $a } };

{ UserInformationProto, effective_user, $a:tt } => { pbdbf!{ get_effectiveUser, set_effectiveUser, $a } };

{ RequestHeaderProto, method_name, $a:tt } => { pbdbf!{ get_methodName, set_methodName, $a } };
{ RequestHeaderProto, declaring_class_protocol_name, $a:tt } => { pbdbf!{ get_declaringClassProtocolName, set_declaringClassProtocolName, $a } };
{ RequestHeaderProto, client_protocol_version, $a:tt } => { pbdbf!{ get_clientProtocolVersion, set_clientProtocolVersion, $a } };

//Security.proto
// (identifier, password, kind, service)
{ TokenProto, identifier, $a:tt }=> { pbdbf!{ get_identifier, set_identifier, $a } };
{ TokenProto, password, $a:tt }=> { pbdbf!{ get_password, set_password, $a } };
{ TokenProto, kind, $a:tt }=> { pbdbf!{ get_kind, set_kind, $a } };
{ TokenProto, service, $a:tt }=> { pbdbf!{ get_service, set_service, $a } };

//Datatransfer
{ OpReadBlockProto, header, $a:tt } => { pbdbf!{ get_header, set_header, $a } };
{ OpReadBlockProto, offset, $a:tt } => { pbdbf!{ get_offset, set_offset, $a } };
{ OpReadBlockProto, len, $a:tt }=> { pbdbf!{ get_len, set_len, $a } };

// (header, targets, source, stage, pipeline_size, min_bytes_rcvd, max_bytes_rcvd, latest_generation_stamp, requested_checksum, caching_strategy, storage_type, target_storage_types, allow_lazy_persist)
{ OpWriteBlockProto, header, $a:tt }=> { pbdbf!{ get_header, set_header, $a } };
{ OpWriteBlockProto, targets, $a:tt }=> { pbdbf_rep!{ get_targets, set_targets, $a } };
{ OpWriteBlockProto, source, $a:tt }=> { pbdbf!{ get_source, set_source, $a } };
{ OpWriteBlockProto, stage, $a:tt }=> { pbdbf!{ get_stage, set_stage, $a } };
{ OpWriteBlockProto, pipeline_size, $a:tt }=> { pbdbf!{ get_pipelineSize, set_pipelineSize, $a } };
{ OpWriteBlockProto, min_bytes_rcvd, $a:tt }=> { pbdbf!{ get_minBytesRcvd, set_minBytesRcvd, $a } };
{ OpWriteBlockProto, max_bytes_rcvd, $a:tt }=> { pbdbf!{ get_maxBytesRcvd, set_maxBytesRcvd, $a } };
{ OpWriteBlockProto, latest_generation_stamp, $a:tt }=> { pbdbf!{ get_latestGenerationStamp, set_latestGenerationStamp, $a } };
{ OpWriteBlockProto, requested_checksum, $a:tt }=> { pbdbf!{ get_requestedChecksum, set_requestedChecksum, take_requestedChecksum, $a } };
{ OpWriteBlockProto, caching_strategy, $a:tt }=> { pbdbf!{ get_cachingStrategy, set_cachingStrategy, $a } };
{ OpWriteBlockProto, storage_type, $a:tt }=> { pbdbf!{ get_storageType, set_storageType, $a } };
{ OpWriteBlockProto, target_storage_types, $a:tt }=> { pbdbf!{ get_targetStorageTypes, set_targetStorageTypes, $a } };
{ OpWriteBlockProto, allow_lazy_persist, $a:tt }=> { pbdbf!{ get_allowLazyPersist, set_allowLazyPersist, $a } };

{ ExtendedBlockProto, pool_id, $a:tt } => { pbdbf!{ get_poolId, set_poolId, take_poolId, $a } };
{ ExtendedBlockProto, block_id, $a:tt } => { pbdbf!{ get_blockId, set_blockId, $a } };
{ ExtendedBlockProto, generation_stamp, $a:tt } => { pbdbf!{ get_generationStamp, set_generationStamp, $a } };
{ ExtendedBlockProto, num_bytes, $a:tt } => { pbdbf!{ get_numBytes, set_numBytes, $a } };

// (status, first_bad_link, checksum_response, read_op_checksum_info, message, short_circuit_access_version)
{ BlockOpResponseProto, status, $a:tt }=> { pbdbf!{ get_status, set_status, $a } };
{ BlockOpResponseProto, first_bad_link, $a:tt }=> { pbdbf!{ get_firstBadLink, set_firstBadLink, take_firstBadLink, $a } };
{ BlockOpResponseProto, checksum_response, $a:tt }=> { pbdbf!{ get_checksumResponse, set_checksumResponse, $a } };
{ BlockOpResponseProto, read_op_checksum_info, $a:tt }=> { pbdbf!{ get_readOpChecksumInfo, set_readOpChecksumInfo, take_readOpChecksumInfo, $a } };
{ BlockOpResponseProto, message, $a:tt }=> { pbdbf!{ get_message, set_message, take_message, $a } };
{ BlockOpResponseProto, short_circuit_access_version, $a:tt }=> { pbdbf!{ get_shortCircuitAccessVersion, set_shortCircuitAccessVersion, $a } };

{ ClientOperationHeaderProto, client_name, $a:tt} => { pbdbf!{ get_clientName, set_clientName, $a } };
{ ClientOperationHeaderProto, base_header, $a:tt} => { pbdbf!{ get_baseHeader, set_baseHeader, $a } };

{ PacketHeaderProto, offset_in_block, $a:tt} => { pbdbf!{ get_offsetInBlock, set_offsetInBlock, $a } };
{ PacketHeaderProto, seqno, $a:tt} => { pbdbf!{ get_seqno, set_seqno, $a } };
{ PacketHeaderProto, last_packet_in_block, $a:tt} => { pbdbf!{ get_lastPacketInBlock, set_lastPacketInBlock, $a } };
{ PacketHeaderProto, data_len, $a:tt} => { pbdbf!{ get_dataLen, set_dataLen, $a } };

{ BaseHeaderProto, block, $a:tt} => { pbdbf!{ get_block, set_block, $a } };
{ BaseHeaderProto, token, $a:tt} => { pbdbf!{ get_token, set_token, $a } };

{ ClientReadStatusProto, status, $a:tt } => { pbdbf!{ get_status, set_status, $a } };

{ ReadOpChecksumInfoProto, checksum, $a:tt } => { pbdbf!{ get_checksum, set_checksum, take_checksum, $a } };
{ ReadOpChecksumInfoProto, chunk_offset, $a:tt } => { pbdbf!{ get_chunkOffset, set_chunkOffset, $a } };

{ ChecksumProto, type, $a:tt } => { pbdbf!{ get_field_type, set_field_type, $a } };
{ ChecksumProto, bytes_per_checksum, $a:tt } => { pbdbf!{ get_bytesPerChecksum, set_bytesPerChecksum, $a } };

// (seqno, status, downstream_ack_time_nanos)
{ PipelineAckProto, seqno, $a:tt }=> { pbdbf!{ get_seqno, set_seqno, $a } };
{ PipelineAckProto, status, $a:tt }=> { pbdbf!{ get_status, set_status, $a } };
{ PipelineAckProto, downstream_ack_time_nanos, $a:tt }=> { pbdbf!{ get_downstreamAckTimeNanos, set_downstreamAckTimeNanos, $a } };
}

#[macro_export]
macro_rules! pbdbf {
    { $get:ident, $set:ident, (get($r:expr)) } => { $r.$get() };
    { $get:ident, $set:ident, (take($r:expr)) } => { $r.$get() };
    { $get:ident, $set:ident, $take:ident, (get($r:expr)) } => { $r.$get() };
    { $get:ident, $set:ident, $take:ident, (take($r:expr)) } => { $r.$take() };
    { $get:ident, $set:ident, (set($r:expr, $v:expr)) } => { $r.$set($v) };
    { $get:ident, $set:ident, $take:ident, (set($r:expr, $v:expr)) } => { $r.$set($v) };
}

//`repeated` field
#[macro_export]
macro_rules! pbdbf_rep {
    { $get:ident, $set:ident, (get($r:expr)) } => { $r.$get() };
    { $get:ident, $set:ident, (take($r:expr)) } => { $r.$get() };
    { $get:ident, $set:ident, $take:ident, (get($r:expr)) } => { $r.$get() };
    { $get:ident, $set:ident, $take:ident, (take($r:expr)) } => { $r.$take().to_vec() };
    { $get:ident, $set:ident, (set($r:expr, $v:expr)) } => { $r.$set(protobuf::RepeatedField::from_vec($v)) };
    { $get:ident, $set:ident, $take:ident, (set($r:expr, $v:expr)) } => { $r.$set(protobuf::RepeatedField::from_vec($v)) };
}

#[macro_export]
macro_rules! pb_cons {
    { $t:ident, $($f:ident : $v:expr),* } => {
        {
            let mut r = <$t>::new();
            $(
                pbdb!{$t, $f, (set(r, $v))}
            )*
            r
        }
    }
}

#[macro_export]
macro_rules! pb_decons {
    { $t:ident, $r:expr, $f:ident } => {
        {
            #[allow(unused_mut)]
            let mut x = $r;
            pbdb!{$t, $f, (take(x))}
        }
    };
    { $t:ident, $r:expr, $($f:ident),+ } => {
        {
            #[allow(unused_mut)]
            let mut x = $r;
            (
                $(
                    pbdb!{$t, $f, (take(x))}
                ),+
            )
        }
    };
}

#[macro_export]
macro_rules! pb_get {
    { $t:ident, $r:expr, $f:ident } => {
        pbdb!{$t, $f, (get($r)) }
    };
    { $t:ident, $r:expr, $($f:ident),+ } => {
        (
            $(
                pbdb!{$t, $f, (get($r)) }
            ),+
        )
    };
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


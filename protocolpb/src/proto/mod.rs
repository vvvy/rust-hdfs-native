pub mod hadoop {
    pub mod GenericRefreshProtocol;
    pub mod GetUserMappingsProtocol;
    pub mod HAServiceProtocol;
    pub mod IpcConnectionContext;
    pub mod ProtobufRpcEngine;
    pub mod ProtocolInfo;
    pub mod RefreshAuthorizationPolicyProtocol;
    pub mod RefreshCallQueueProtocol;
    pub mod RefreshUserMappingsProtocol;
    pub mod RpcHeader;
    pub mod Security;
    pub mod TraceAdmin;
    pub mod ZKFCProtocol;
}


pub mod hdfs {
    pub use super::hadoop::Security;

    pub mod acl;
    pub mod ClientDatanodeProtocol;
    pub mod ClientNamenodeProtocol;
    pub mod DatanodeProtocol;
    pub mod datatransfer;
    pub mod encryption;
    pub mod fsimage;
    pub mod HAZKInfo;
    pub mod hdfs;
    pub mod inotify;
    pub mod InterDatanodeProtocol;
    pub mod JournalProtocol;
    pub mod NamenodeProtocol;
    pub mod QJournalProtocol;
    pub mod xattr;
}



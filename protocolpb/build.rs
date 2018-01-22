/// In order to compile protobuf sources, define env var `HDFS_PROTOC_PATH` to point at a directory
/// where `protoc` (`protoc.exe` under Windows) executable can be found.

fn main() {
    extern crate protoc_rust;

    use std::env;
    use std::path::PathBuf;

    if let (Some(path), Some(protoc_path)) = (env::var_os("PATH"), env::var_os("HDFS_PROTOC_PATH")) {
        let mut paths = env::split_paths(&path).collect::<Vec<_>>();
        paths.push(PathBuf::from(protoc_path));
        let new_path = env::join_paths(paths).unwrap();
        env::set_var("PATH", &new_path);

        protoc_rust::run(protoc_rust::Args {
            out_dir: "src/proto",
            input: &[
                "proto/acl.proto",
                "proto/ClientDatanodeProtocol.proto",
                "proto/ClientNamenodeProtocol.proto",
                "proto/DatanodeProtocol.proto",
                "proto/datatransfer.proto",
                "proto/encryption.proto",
                "proto/fsimage.proto",
                "proto/GenericRefreshProtocol.proto",
                "proto/GetUserMappingsProtocol.proto",
                "proto/HAServiceProtocol.proto",
                "proto/HAZKInfo.proto",
                "proto/hdfs.proto",
                "proto/inotify.proto",
                "proto/InterDatanodeProtocol.proto",
                "proto/IpcConnectionContext.proto",
                "proto/JournalProtocol.proto",
                "proto/NamenodeProtocol.proto",
                "proto/ProtobufRpcEngine.proto",
                "proto/ProtocolInfo.proto",
                "proto/QJournalProtocol.proto",
                "proto/RefreshAuthorizationPolicyProtocol.proto",
                "proto/RefreshCallQueueProtocol.proto",
                "proto/RefreshUserMappingsProtocol.proto",
                "proto/RpcHeader.proto",
                "proto/Security.proto",
                "proto/TraceAdmin.proto",
                "proto/xattr.proto",
                "proto/ZKFCProtocol.proto",
            ],
            includes: &["proto"],
        }).expect("protoc");
    }


    
    /*
    extern crate protoc_rust_grpc;

    protoc_rust_grpc::run(protoc_rust_grpc::Args {
        out_dir: "src/proto",
        includes: &["proto"],
        input: &[
            "proto/acl.proto",
            "proto/ClientDatanodeProtocol.proto",
            "proto/ClientNamenodeProtocol.proto",
            "proto/DatanodeProtocol.proto",
            "proto/datatransfer.proto",
            "proto/encryption.proto",
            "proto/fsimage.proto",
            "proto/HAZKInfo.proto",
            "proto/hdfs.proto",
            "proto/inotify.proto",
            "proto/InterDatanodeProtocol.proto",
            "proto/JournalProtocol.proto",
            "proto/NamenodeProtocol.proto",
            "proto/QJournalProtocol.proto",
            "proto/xattr.proto",
            "proto/Security.proto"
        ],
        rust_protobuf: true, // also generate protobuf messages, not just services
    }).expect("protoc-rust-grpc");
    */
}

/// In order to compile protobuf sources, define env var `HDFS_PROTOC_PATH` to point at a directory
/// where `protoc` (`protoc.exe` under Windows) executable can be found.

fn main() {
    extern crate protoc_rust;

    use std::env;
    use std::path::PathBuf;
    use std::fs;

    if let (Some(path), Some(protoc_path)) = (env::var_os("PATH"), env::var_os("HDFS_PROTOC_PATH")) {
        let mut paths = env::split_paths(&path).collect::<Vec<_>>();
        paths.push(PathBuf::from(protoc_path));
        let new_path = env::join_paths(paths).unwrap();
        env::set_var("PATH", &new_path);


        let hadoop_target_dir = "src/proto/hadoop";
        let hadoop_source_dir = "proto/hadoop";
        fs::create_dir_all(PathBuf::from(hadoop_target_dir)).expect("mkdir hadoop_target_dir");
        protoc_rust::run(protoc_rust::Args {
            out_dir: hadoop_target_dir,
            input: &[
                "proto/hadoop/GenericRefreshProtocol.proto",
                "proto/hadoop/GetUserMappingsProtocol.proto",
                "proto/hadoop/HAServiceProtocol.proto",
                "proto/hadoop/IpcConnectionContext.proto",
                "proto/hadoop/ProtobufRpcEngine.proto",
                "proto/hadoop/ProtocolInfo.proto",
                "proto/hadoop/RefreshAuthorizationPolicyProtocol.proto",
                "proto/hadoop/RefreshCallQueueProtocol.proto",
                "proto/hadoop/RefreshUserMappingsProtocol.proto",
                "proto/hadoop/RpcHeader.proto",
                "proto/hadoop/Security.proto",
                "proto/hadoop/TraceAdmin.proto",
                "proto/hadoop/ZKFCProtocol.proto",
            ],
            includes: &[hadoop_source_dir],
        }).expect("protoc (hadoop)");

        let hdfs_target_dir = "src/proto/hdfs";
        let hdfs_source_dir = "proto/hdfs";
        fs::create_dir_all(PathBuf::from(hdfs_target_dir)).expect("mkdir hdfs_target_dir");
        protoc_rust::run(protoc_rust::Args {
            out_dir: hdfs_target_dir,
            input: &[
                "proto/hdfs/acl.proto",
                "proto/hdfs/ClientDatanodeProtocol.proto",
                "proto/hdfs/ClientNamenodeProtocol.proto",
                "proto/hdfs/DatanodeProtocol.proto",
                "proto/hdfs/datatransfer.proto",
                "proto/hdfs/encryption.proto",
                "proto/hdfs/fsimage.proto",
                "proto/hdfs/HAZKInfo.proto",
                "proto/hdfs/hdfs.proto",
                "proto/hdfs/inotify.proto",
                "proto/hdfs/InterDatanodeProtocol.proto",
                "proto/hdfs/JournalProtocol.proto",
                "proto/hdfs/NamenodeProtocol.proto",
                "proto/hdfs/QJournalProtocol.proto",
                "proto/hdfs/xattr.proto",
            ],
            includes: &[hadoop_source_dir, hdfs_source_dir],
        }).expect("protoc (hdfs)");
        
    }
}

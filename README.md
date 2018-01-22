# rust-hdfs-native

Native Rust HDFS client library. Talks directly to Hadoop NN/DNs over Hadoop RPC.

## Building with `protobuf` crate

Prereq: download protoc compiler v 2.5.0, e.g. from here: `https://repo1.maven.org/maven2/com/google/protobuf/protoc/2.5.0/`. For Win10 64, `protoc-2.5.0-windows-x86_64.exe` is Ok. Make the executable available in PATH as `protoc.exe` (win) or `protoc`.

NOTE: this is a Work In Progress
use rpc_nn;
use error;
use ::Result;
use std;
use std::borrow::Cow;

use protocolpb::proto::hdfs::ClientNamenodeProtocol::{
    GetListingRequestProto, GetListingResponseProto
};

use protocolpb::proto::hdfs::hdfs::{HdfsFileStatusProto_FileType};

//path: String, start_from: String, max_count: u32
pub fn read_dir_listing(c: rpc_nn::Connection, st: rpc_nn::ConnectionState) -> Result<()> {

    let args: Vec<String> = std::env::args().skip(2).collect();

    let src = match args.len() {
        1 => Ok(args[0].clone()),
        _ => Err(error::Error::Other(Cow::from("invalid command line")))
    }?;

    let mut q = GetListingRequestProto::new();
    q.set_src(src);
    q.set_startAfter(vec![]);
    q.set_needLocation(false);
    let (_, r) = c.call::<_, GetListingResponseProto>(st, "getListing".to_owned(), q)?;

    for fs in r.get_dirList().get_partialListing() {
        let sz = match fs.get_fileType() {
            HdfsFileStatusProto_FileType::IS_DIR => format!("<dir, {} entries>", fs.get_childrenNum()),
            HdfsFileStatusProto_FileType::IS_SYMLINK => format!("->{}", String::from_utf8_lossy(fs.get_symlink())),
            _ => format!("{}", fs.get_length())
        };
        println!("{}\t{}", String::from_utf8_lossy(fs.get_path()), sz);
    }
    trace!("RESULT: {:?}", r);
    Ok(())
}



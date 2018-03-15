
//use std;
//use std::borrow::Cow;
use std::net::ToSocketAddrs;
use tokio_core::reactor::Core;
use protobuf_api::*;
use *;

use nn::*;
use futures::{Future};
use futures::future::ok;

//path: String, start_from: String, max_count: u32
pub fn read_dir_listing(args: config::GetListing, cfg: &config::Common) -> Result<()> {

    let mut core = Core::new()?;

    let addr = "127.0.0.1:8020".to_socket_addrs()?.next().ok_or(app_error!(other "NN host not found"))?;

    let mut c = nn::Connection::connect(
        &core.handle(),
        &addr,
        "cloudera".to_owned()
    );

    for src in args.src {
        let mut q = GetListingRequestProto::new();
        q.set_src(src);
        q.set_startAfter(vec![]);
        q.set_needLocation(false);

        let f =
            c.and_then(|conn| conn.call(NnQ::GetListing(q)));

        let (nr, c1) = core.run(f)?;
        c = Box::new(ok(c1));

        let r = match nr {
            NnR::GetListing(glr) => Ok(glr),
            o => Err(app_error!(other "unexpected response: expected `NnR::GetListing` but got {:?}", o))
        }?;

        for fs in r.get_dirList().get_partialListing() {
            let sz = match fs.get_fileType() {
                HdfsFileStatusProto_FileType::IS_DIR => format!("<dir, {} entries>", fs.get_childrenNum()),
                HdfsFileStatusProto_FileType::IS_SYMLINK => format!("->{}", String::from_utf8_lossy(fs.get_symlink())),
                _ => format!("{}", fs.get_length())
            };
            println!("{}\t{}", String::from_utf8_lossy(fs.get_path()), sz);
        }
        trace!("RESULT: {:?}", r);
    }

    Ok(())
}

pub fn read_block() -> Result<()> { unimplemented!() }

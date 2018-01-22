#[macro_use] extern crate log;

extern crate futures;
extern crate native_tls;
extern crate tokio_core;
extern crate tokio_io;

extern crate protobuf;
extern crate byteorder;
extern crate protocolpb;

mod error;
mod rpc_codec;
mod rpc;
mod hdfs;
mod cmdln;

use std::net::ToSocketAddrs;
use tokio_core::reactor::Core;

type Result<T> = std::result::Result<T, error::Error>;

fn do_things<F: FnOnce(rpc::Connection, rpc::ConnectionState) -> Result<()>>(f: F) -> Result<()> {
    let core = Core::new()?;

    let addr = "127.0.0.1:8020".to_socket_addrs()?.next().ok_or(error::Error::Other("NN host not found".to_owned()))?;

    let c = rpc::Connection::new(vec![1, 2, 3, 4, 4, 3, 2, 1], "cloudera".to_owned());

    let st = c.connect(core, addr)?;

    f(c, st)?;
    Ok(())
}

fn usage() -> ! {
    println!("USAGE: TODO");
    std::process::exit(1);
}

fn main() {
    let ocmd = std::env::args().nth(1);

    let r = match ocmd {
        Some(ref cmd) => match cmd.as_ref() {
            "ls" => do_things(hdfs::read_dir_listing),
            other => Err(error::Error::Other(format!("Invalid command `{}`", other)))
        },
        None => usage()
    };

    match r {
        Ok(()) => (),
        Err(e) => {
            eprintln!("{}", e);
            std::process::exit(2)
        }
    }
}

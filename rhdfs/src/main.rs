//! Rust native HDFS client

#[macro_use] extern crate log;
extern crate env_logger;

extern crate futures;
extern crate native_tls;
extern crate tokio_core;
extern crate tokio_io;
extern crate tokio_proto;
extern crate tokio_service;

extern crate protobuf;
extern crate byteorder;
extern crate bytes;
extern crate protocolpb;

#[macro_use] mod error;
mod result;
mod codec_nn;
mod codec_dt;
mod rpc_nn;
mod rpc_dt;
mod protocol_dt;
mod hdfs;
mod cmdln;
mod codec_tools;
mod protobuf_api;
mod util;

use std::borrow::Cow;
use std::net::ToSocketAddrs;
use tokio_core::reactor::Core;

pub use error::*;
pub use result::Result;



fn do_things<F: FnOnce(rpc_nn::Connection, rpc_nn::ConnectionState) -> Result<()>>(f: F) -> Result<()> {
    let core = Core::new()?;

    let addr = "127.0.0.1:8020".to_socket_addrs()?.next().ok_or(error::Error::Other(Cow::from("NN host not found")))?;

    let c = rpc_nn::Connection::new(vec![1, 2, 3, 4, 4, 3, 2, 1], "cloudera".to_owned());

    let st = c.connect(core, addr)?;

    f(c, st)?;
    Ok(())
}

fn usage() -> ! {
    println!("USAGE: TODO");
    std::process::exit(1);
}

fn main() {
    env_logger::init();
    trace!("Tracing started");
    let ocmd = std::env::args().nth(1);

    let r = match ocmd {
        Some(ref cmd) => match cmd.as_ref() {
            "ls" => do_things(hdfs::read_dir_listing),
            "dt" => codec_dt::do_things(),
            other => Err(error::Error::Other(Cow::from(format!("Invalid command `{}`", other))))
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

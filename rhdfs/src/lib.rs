//! Rust native HDFS client

#[macro_use] extern crate log;

extern crate futures;
extern crate native_tls;
extern crate tokio_core;
extern crate tokio_io;
extern crate protobuf;
extern crate byteorder;
extern crate bytes;
extern crate crc;
extern crate protocolpb;

#[macro_use] mod error;
#[macro_use] mod protobuf_api;
mod result;
mod codec_nn;
mod rpc_nn;
mod hdfs;
mod codec_tools;
mod util;
mod dt;

use std::borrow::Cow;
use std::net::ToSocketAddrs;
use tokio_core::reactor::Core;

pub use error::*;
pub use result::Result;



fn do_things<F: FnOnce(rpc_nn::Connection, rpc_nn::ConnectionState) -> Result<()>>(f: F) -> Result<()> {
    let core = Core::new()?;

    let addr = "127.0.0.1:8020".to_socket_addrs()?.next().ok_or(Error::Other(Cow::from("NN host not found")))?;

    let c = rpc_nn::Connection::new(vec![1, 2, 3, 4, 4, 3, 2, 1], "cloudera".to_owned());

    let st = c.connect(core, addr)?;

    f(c, st)?;
    Ok(())
}


pub fn ls() -> Result<()> { do_things(hdfs::read_dir_listing) }

pub fn read_block() -> Result<()> { unimplemented!() }
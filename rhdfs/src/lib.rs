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
extern crate uuid;
extern crate protocolpb;

#[macro_use] mod util;
#[macro_use] mod error;
#[macro_use] mod protobuf_api;
mod result;
mod codec_tools;
mod dt;
mod nn;

pub mod hdfs;
pub mod config;

pub use error::*;
pub use result::*;

pub use std::io::Error as IoError;
pub use std::io::Result as IoResult;

//pub fn ls() -> Result<()> { hdfs::read_dir_listing() }

//pub fn read_block() -> Result<()> { unimplemented!() }
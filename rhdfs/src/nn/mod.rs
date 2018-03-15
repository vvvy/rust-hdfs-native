//! Namenode protocol implementation (standard port 8020)

mod codec;
mod proto;

pub use self::proto::Connection;
pub use self::codec::{RpcReq, NnQ, NnR};
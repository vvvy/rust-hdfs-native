use std::error::Error as StdError;
//use std::result::Result as StdResult;
use std::fmt::{Display, Formatter, Result};

use protobuf::ProtobufError;
use std::io::Error as IoError;
#[derive(Debug)]
pub enum Error {
    Protobuf(ProtobufError),
    IO(IoError),
    RPC { protocol: String, status: String, message: String, code: String, exception_class: String },
    Other(String)
}

impl StdError for Error {
    fn description(&self) -> &str {
        match self {
            &Error::Protobuf(ref pe) => pe.description(),
            &Error::IO(ref io) => io.description(),
            &Error::RPC { ref message, protocol:_, status: _, code: _, exception_class: _ } => message,
            &Error::Other(ref s) => s
        }
    }

    fn cause(&self) -> Option<&StdError> {
        match self {
            &Error::Protobuf(ref pe) => pe.cause(),
            &Error::IO(ref io) => io.cause(),
            &Error::RPC { message: _, protocol:_, status: _, code: _, exception_class: _} => None,
            &Error::Other(_) => None
        }
    }
}

impl Display for Error {
    fn fmt(&self, f: &mut Formatter) -> Result {
        match self {
            &Error::Protobuf(ref pe) =>
                write!(f, "ProtobufError: {}", pe),
            &Error::IO(ref io) =>
                write!(f, "IoError: {}", io),
            &Error::RPC { ref protocol, ref status, ref message, ref code, ref exception_class } =>
                write!(f, "RpcError(protocol={}, status={}, message={}, code={}, exception_class={})",
                       protocol, status, message, code, exception_class),
            &Error::Other(ref s) =>
                write!(f, "Error: {}", s)
        }
    }
}

impl From<ProtobufError> for Error {
    fn from(e: ProtobufError) -> Self {
        Error::Protobuf(e)
    }
}

impl From<IoError> for Error {
    fn from(e: IoError) -> Self {
        Error::IO(e)
    }
}

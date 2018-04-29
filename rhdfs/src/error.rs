
use std::fmt::{Display, Formatter, Result};
pub use std::borrow::Cow;
use protobuf::ProtobufError;
use protocolpb::proto::hdfs::datatransfer::Status as DtStatus;
use protocolpb::proto::hadoop::RpcHeader::{
    RpcResponseHeaderProto_RpcStatusProto as RpcStatusProto,
    RpcResponseHeaderProto_RpcErrorCodeProto as RpcErrorCodeProto
};
use std::io::ErrorKind;
use *;

#[derive(Debug)]
pub enum Error {
    Protobuf(ProtobufError),
    IO(IoError),
    RPC {
        protocol: String,
        status: RpcStatusProto,
        error_detail: RpcErrorCodeProto,
        error_msg: String,
        exception_class_name: String
    },
    ShortBuffer(usize),
    Codec(Cow<'static, str>),
    Namenode(Cow<'static, str>),
    DataTransfer(DtStatus, String),
    Other(Cow<'static, str>)
}


#[macro_export]
macro_rules! app_error {
    {codec $e:expr, $($es:expr),+} => { Error::Codec(Cow::from(format!($e, $($es),+))) };
    {codec $e:expr} => { Error::Codec(Cow::from($e)) };
    {nn $e:expr, $($es:expr),+} => { Error::Namenode(Cow::from(format!($e, $($es),+))) };
    {nn $e:expr} => { Error::Namenode(Cow::from($e)) };
    {dt $s:expr, $m:expr} => { Error::DataTransfer($s, $m.to_owned()) };
    {unreachable} => { Error::Other(Cow::from("got to an unreachable point in code")) };
    //{other $e:expr, $($es:expr),+} => { Error::Other(Cow::from(format!($e, $($es),+))) };
    {other $e:expr} => { Error::Other(Cow::from($e)) };
    {other $($es:expr),+} => { Error::Other(Cow::from(format!($($es),+))) };
}

impl StdError for Error {
    fn description(&self) -> &str {
        match self {
            &Error::Protobuf(ref pe) => pe.description(),
            &Error::IO(ref io) => io.description(),
            &Error::RPC { ref error_msg, protocol:_, status: _, error_detail: _, exception_class_name: _ } => error_msg,
            &Error::ShortBuffer(_) => "Buffer short",
            &Error::Codec(ref s) => s,
            &Error::Namenode(ref s) => s,
            &Error::DataTransfer(_, ref s) => s,
            &Error::Other(ref s) => s
        }
    }

    fn cause(&self) -> Option<&StdError> {
        match self {
            &Error::Protobuf(ref pe) => pe.cause(),
            &Error::IO(ref io) => io.cause(),
            &Error::RPC { error_msg: _, protocol:_, status: _, error_detail: _, exception_class_name: _} |
                &Error::ShortBuffer(..) |
                &Error::Codec(..) |
                &Error::Namenode(..) |
                &Error::DataTransfer(..) |
                &Error::Other(..)
                    => None
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
            &Error::RPC { ref protocol, ref status, ref error_detail, ref error_msg, ref exception_class_name } =>
                write!(f, "RpcError(protocol={}, status={:?}, error_detail={:?}, error_msg={}, exception_class_name={})",
                       protocol, status, error_detail, error_msg, exception_class_name),
            &Error::ShortBuffer(n) =>
                write!(f, "Buffer short by({})", n),
            &Error::Codec(ref s) =>
                write!(f, "CodecError: {}", s),
            &Error::Namenode(ref m) =>
                write!(f, "NameNodeError: `{}`", m),
            &Error::DataTransfer(ref s, ref m) =>
                write!(f, "DataTransferError: {:?} `{}`", s, m),
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

impl From<Error> for IoError {
    fn from(e: Error) -> Self {
        match e {
            Error::IO(io) => io,
            other => IoError::new(ErrorKind::Other, other)
        }
    }
}

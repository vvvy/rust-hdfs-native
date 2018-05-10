pub use std::io::Error as IoError;
pub use std::io::Result as IoResult;
pub use std::error::Error as StdError;
pub use std::result::Result as StdResult;

use futures::Future;

pub type FI<T> = Future<Item=T, Error=IoError>;
pub type BFI<T> = Box<Future<Item=T, Error=IoError> + Send>;
pub type BF<T, U> = Box<Future<Item=T, Error=U> + Send>;
pub type BFTT<T> = BF<T, T>;


pub use std::io::Error as IoError;
pub use std::io::Result as IoResult;
pub use std::error::Error as StdError;
pub use std::result::Result as StdResult;

use futures::Future;

pub type FI<T> = Future<Item=T, Error=IoError>;
pub type BFI<T> = Box<Future<Item=T, Error=IoError> + Send>;
pub type BF<T, U> = Box<Future<Item=T, Error=U> + Send>;
pub type BFTET<T> = BF<T, (::error::Error, T)>;

pub trait BiMap<T> {
    fn bimap<C, U>(self, C) -> BFTET<U> where
        C: FnOnce(T) -> U + Send +'static,
        U: Send + 'static;
}

impl<F, T> BiMap<T> for F where F: Future<Item=T, Error=(::error::Error, T)> + Send + 'static {
    fn bimap<C, U>(self, c: C) -> BFTET<U> where
        C: FnOnce(T) -> U + Send +'static,
        U: Send + 'static {
        Box::new(self.then(move |a| match a {
            Ok(t) => Ok(c(t)),
            Err((e, t)) => Err((e, c(t)))
        }))
    }
}

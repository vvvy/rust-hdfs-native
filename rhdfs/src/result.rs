use ::Error;
use *;

pub type Result<T> = StdResult<T, Error>;


pub trait ErrorConverter<T> {
    fn c_err(self) -> T;
}

impl<T, E> ErrorConverter<StdResult<T, E>> for Result<T> where E: From<Error> {
    fn c_err(self) -> StdResult<T, E> {
        self.map_err(|e| E::from(e))
    }
}
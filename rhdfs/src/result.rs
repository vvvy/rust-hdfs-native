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

pub trait Commute<T> {
    fn commute(self) -> (T, Result<()>);
}

impl<T> Commute<T> for StdResult<T, (Error, T)> {
    fn commute(self) -> (T, Result<()>) {
        match self {
            Ok(t) => (t, Ok(())),
            Err((e, t)) => (t, Err(e))
        }
    }
}

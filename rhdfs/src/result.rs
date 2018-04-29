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

pub fn result_from_errors<E: Into<Error> + std::fmt::Display>(mut errs: Vec<E>) -> Result<()> {
    let e = errs.pop();
    e.map_or(
        Ok(()),
        |e| Err(
            if errs.is_empty() {
                e.into()
            } else {
                app_error!(other "Multiple errors found, the last is: {}", e)
            }
        )
    )
}
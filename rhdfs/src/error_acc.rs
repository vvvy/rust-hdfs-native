use futures::future::{Future, err};
use *;

pub trait ErrorAccumulator {
    fn error(self, e: IoError) -> Self;
}

pub fn b_accumulate<P,T>(p: P, e: IoError) -> BF<T, P> where
    P: ErrorAccumulator + Send + 'static,
    T: Send + 'static
{
    Box::new(err(p.error(e)))
}

pub fn bimap<T, U, F, C>(f: F, c: C) -> BFTT<U> where
    F: Future<Item=T, Error=T> + Send + 'static,
    C: FnOnce(T) -> U + Send +'static,
    U: Send + 'static {
    Box::new(f.then(move |a| match a {
        Ok(t) => Ok(c(t)),
        Err(t) => Err(c(t))
    }))
}

pub fn biunwrap<R>(r: StdResult<R, R>) -> (bool, R) {
    (r.is_ok(), match r { Ok(r) | Err(r) => r })
}

pub trait ErrorExtractor where Self: Sized {
    fn extract_errors(self) -> (Vec<IoError>, Self);

    fn from_result(r: StdResult<Self, Self>) ->  (Vec<IoError>, Self) {
        let ok = r.is_ok();
        let r = match r { Ok(r) | Err(r) => r };
        let (mut ve, s) = r.extract_errors();
        if !ok && ve.is_empty() { ve.push(app_error!(other "Unknown error").into())}
        (ve, s)
    }
}

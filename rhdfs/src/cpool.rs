use std::collections::{VecDeque, HashMap};
use std::collections::hash_map::Entry;
use std::hash::Hash;
use std::time::{Instant, Duration};
use std::marker::PhantomData;
use std::fmt::Debug;

use futures::{Future, Stream, Sink};
use futures::future::{ok, err};
use futures::sync::*;

use *;


/// Connection pool client abstraction. Must be easily cloneable. `K` is key type,
/// `A` is connection address, and `C` is connection type
pub trait ConnectionPool<K, A, C> {
    /// given a key, conn data, and a closure producing future of evaluation result `R` using
    /// connection 'C', return a future of that 'R'. If no conn data present, try to use cached or
    /// preconfigured data for this `K`
    fn exec<F, R>(&self, key: K, addr: Option<A>, f: F) -> BFI<R> where
        R: Send + 'static,
        F: FnOnce(C) -> BFI<(C, R)> + Send + 'static;

    /// run a protocol fsm object 'r' through a connection. 'r' accumulates connection errors among
    /// other errors.
    fn run<F, R>(&self, key: K, addr: Option<A>, r: R, f: F) -> BF<R, R> where
        R: Send + ErrorAccumulator + 'static,
        F: FnOnce((C, R)) -> BF<(C, R), R> + Send + 'static;
}

pub trait Connector<A, C> {
    fn connect(&mut self, a: A) -> BFI<C>;
}

#[derive(Debug)]
enum CPCmd<K, A, C> {
    Borrow(K, Option<A>, oneshot::Sender<IoResult<C>>),
    Return(K, C)
}

pub trait AgeChecker {
    fn is_ok(&self, t: Instant) -> bool;
}

pub trait AgeCheckerFactory {
    type E: AgeChecker;
    fn now() -> Instant;
    fn checker(&self) -> Self::E;
}

pub struct AgeCheckerImpl {
    threshold: Instant
}

impl AgeChecker for AgeCheckerImpl {
    fn is_ok(&self, t: Instant) -> bool {
        t > self.threshold
    }
}

pub struct AgeCheckerFactoryImpl {
    max_age: Duration
}

impl AgeCheckerFactoryImpl {
    pub fn new(max_age: Duration) -> AgeCheckerFactoryImpl {
        AgeCheckerFactoryImpl { max_age }
    }
}

impl AgeCheckerFactory for AgeCheckerFactoryImpl {
    type E = AgeCheckerImpl;

    fn now() -> Instant {
        Instant::now()
    }

    fn checker(&self) -> AgeCheckerImpl {
        AgeCheckerImpl { threshold: Instant::now() - self.max_age }
    }
}

#[derive(Debug)]
struct CPValue<A, C> {
    /// Cached address
    a: Option<A>,
    /// The queue
    q: VecDeque<(C, Instant)>
}

#[derive(Debug)]
enum CPResult<A, C> {
    Connection(C),
    Address(A),
    Nothing
}

impl<A, C> CPValue<A, C> where A: Clone {
    fn new() -> CPValue<A, C> { CPValue { a: None, q: VecDeque::new() } }
    fn new_a(a: A) -> CPValue<A, C> { CPValue { a: Some(a), q: VecDeque::new() } }
    ///Update cached a
    fn update_a(&mut self, oa: Option<A>) {
        match oa {
            a @ Some(..) => self.a = a,
            None => ()
        }
    }

    #[inline]
    fn a_cpr(&self) -> CPResult<A, C>  {
        match self.a {
            Some(ref a) => CPResult::Address(a.clone()),
            None => CPResult::Nothing
        }
    }
    fn dequeue<AC: AgeChecker>(&mut self, ac: AC, oa: Option<A>) -> CPResult<A, C> {
        self.update_a(oa);
        loop {
            match self.q.pop_front() {
                Some((c, t)) =>
                    if ac.is_ok(t) { break CPResult::Connection(c) }
                None =>
                    break self.a_cpr()
            }
        }
    }
    fn enqueue(&mut self, c: C, t: Instant) {
        self.q.push_back((c, t))
    }
}


/// CP impl that can run on a separate thread. `C` is connection, and `O` is connector type
#[derive(Debug)]
pub struct ConnectionPoolServer<K, A, C, O, T> where
    K: Eq + Hash
{
    /// The pool itself
    m: HashMap<K, CPValue<A, C>>,
    /// connector
    o: O,
    /// AgeChecker
    t: T,
    a_type: PhantomData<A>
}

impl<K, A, C, O, T> ConnectionPoolServer<K, A, C, O, T> where
    K: Eq + Hash + Debug + Send + 'static,
    A: Clone + Send + 'static,
    C: Send + 'static,
    O: Connector<A, C> + Send + 'static,
    T: AgeCheckerFactory + Send + 'static
{
    pub fn new(o: O, t: T, init_a: Vec<(K, A)>) -> ConnectionPoolServer<K, A, C, O, T> {
        let mut rv = ConnectionPoolServer {
            m: HashMap::new(),
            o,
            t,
            a_type: PhantomData
        };

        for (k, v) in init_a {
            rv.set_a(k, v)
        }

        rv
    }

    fn pop(&mut self, k: K, a: Option<A>) -> StdResult<CPResult<A, C>, K> {
        match (self.m.entry(k), a) {
            (Entry::Vacant(e), None) =>
                Err(e.into_key()),
            (Entry::Vacant(e), Some(a)) =>
                Ok(e.insert(CPValue::new_a(a)).a_cpr()),
            (Entry::Occupied(mut e), oa)  =>
                Ok(e.get_mut().dequeue(self.t.checker(), oa))
        }
    }

    fn set_a(&mut self, k: K, a: A) {
        match self.m.entry(k) {
            Entry::Vacant(e) =>
                drop(e.insert(CPValue::new_a(a))),
            Entry::Occupied(mut e) =>
                e.get_mut().update_a(Some(a))
        }
    }

    fn push(&mut self, k: K, c: C) {
        self.m.entry(k).or_insert_with(|| CPValue::new()).enqueue(c, T::now())
    }

    fn process_cmd(mut self, w: CPCmd<K, A, C>) -> BFI<Self> {
        match w {
            CPCmd::Borrow(k, a, sk) => {
                let f0: BFI<C> = match self.pop(k, a) {
                    Ok(CPResult::Connection(c)) =>
                        Box::new(ok(c)),
                    Ok(CPResult::Address(a)) =>
                        self.o.connect(a),
                    Ok(CPResult::Nothing) =>
                        Box::new(err(app_error!(other "No available connection address (key unknown)").into())),
                    Err(key) =>
                        Box::new(err(app_error!(other "No available connection address for key `{:?}`", key).into()))
                };
                //TODO do sk.send on either outcome of f0
                Box::new(f0.then(|c| {
                    let r = sk.send(c);
                    if r.is_err() { debug!("cpool.server sink send error (client gone?). Connection discarded") }
                    //we intentionally discard a Result<C,IoE> (error outcome) here
                    Box::new(ok(self))
                })) as BFI<Self>
            },
            CPCmd::Return(k, c) => {
                self.push(k, c);
                Box::new(ok(self))
            }
        }
    }

    fn into_future(self, r: mpsc::Receiver<CPCmd<K, A, C>>) -> BFI<Self> {
        Box::new(r
            .map_err(|_| app_error!(other "cpool.server stream reception error").into())
            .fold(
                self,
                |cps, w| cps.process_cmd(w)
            )
        )
    }

    /*
    /// this differs from `into_future` in that it exposes the argument `r` in the loop,
    /// allowing for closing `r`
    /// not sure if it is really needed
    fn into_future2(self, r: mpsc::Receiver<CPCmd<K, A, C>>) -> BFI<Self> {
        Box::new(r
            .into_future()
            .or_else(|(_, r)| ok((None, r)))
            .and_then(|(ocmd, r)|
            match ocmd {
                Some(w) =>
                    Box::new(
                        self.process_cmd(w).and_then(|cps| cps.into_future2(r))
                    ) as BFI<Self>,
                None =>
                    Box::new(ok(self))
            }
        ))
    }
    */

}

pub struct ConnectionPoolClient<K, A, C> {
    s: mpsc::Sender<CPCmd<K, A, C>>
}

impl<K, A, C> Clone for ConnectionPoolClient<K, A, C> {
    fn clone(&self) -> Self {
        ConnectionPoolClient { s: self.s.clone() }
    }
}

impl<K, A, C> ConnectionPoolClient<K, A, C> where
    K: Send + 'static, A: Send + 'static, C: Send + 'static {

    fn borrow(&self, k: K, a: Option<A>) -> BFI<C> {
        let (s, r) = oneshot::channel();
        Box::new(
            self.s.clone()
                .send(CPCmd::Borrow(k, a, s))
                .map_err(|e| app_error!(other "ConnectionPoolClient: borrow: send error: {:?}", e).into())
                .and_then(|_|
                    r.then(|o| match o {
                        Ok(Ok(c)) => Ok(c),
                        Ok(Err(e)) => Err(e),
                        Err(_) => Err(app_error!(other "ConnectionPoolClient: borrow: send cancelled").into())
                    })
                    //r.map_err(|e|
                    //    app_error!(other "ConnectionPoolClient: borrow: send cancelled: {:?}", e).into()
                    //)
                )
        )
    }


}

impl<K, A, C> ConnectionPool<K, A, C> for ConnectionPoolClient<K, A, C> where
    K: Clone + Send + 'static, A: Send + 'static, C: Send + 'static {

    fn exec<F, R>(&self, key: K, addr: Option<A>, f: F) -> BFI<R> where
        R: Send + 'static,
        F: FnOnce(C) -> BFI<(C, R)> + Send + 'static {

        let s1 = self.s.clone();
        let k = key.clone();
        Box::new(self.borrow(key, addr).and_then(
            |c| f(c).and_then(
                |(c, r)| {
                    s1.send(CPCmd::Return(k, c))
                        .map(|_| r)
                        .map_err(|e| app_error!(other "cpool: sink transmission error: {:?}", e).into())
                }
            )
        ))
    }

    fn run<F, R>(&self, key: K, addr: Option<A>, r: R, f: F) -> BF<R, R> where
        R: Send + ErrorAccumulator + 'static,
        F: FnOnce((C, R)) -> BF<(C, R), R> + Send + 'static {

        let s1 = self.s.clone();
        let k = key.clone();
        let f =
            self.borrow(key, addr).then(|w| match w {
                Ok(c) =>
                    Box::new(f((c, r)).and_then(
                        |(c, r)| {
                            s1.send(CPCmd::Return(k, c))
                                .then(|u| match u {
                                    Ok(_) => Ok(r),
                                    Err(e) => Err(r.error(app_error!(other "cpool: sink transmission error: {:?}", e).into()))
                                })
                        }
                    )) as BF<R, R>,
                Err(e) =>
                    Box::new(err(r.error(e)))
            });

        Box::new(f)
    }
}

pub fn create_connection_pool<K, A, C, O, T>(o: O, t: T, buffer: usize, init_a: Vec<(K, A)>)
    -> (ConnectionPoolClient<K, A, C>, BFI<ConnectionPoolServer<K, A, C, O, T>>) where
    K: Eq + Hash + Debug + Send + 'static,
    A: Clone + Send + 'static,
    C: Send + 'static,
    O: Connector<A, C> + Send + 'static,
    T: AgeCheckerFactory  + Send + 'static {
    let (s, r) = mpsc::channel(buffer);
    let c = ConnectionPoolClient { s };
    let f = ConnectionPoolServer::new(o, t, init_a).into_future(r);
    (c, f)
}


/// Synchronous future runner
pub struct Runner<S> {
    s: Option<BFI<S>>
}

impl<S> Runner<S> {
    pub fn new(s: BFI<S>) -> Runner<S> { Runner { s: Some(s) } }

    /// executes `rf` synchronously.
    pub fn exec_r<R, U>(&mut self, rf: BF<R, U>) -> StdResult<R, U> {
        use futures::future::Either;

        let s = std::mem::replace(&mut self.s, None);

        let (r, s) = match s {
            Some(srv) =>
                match rf.select2(srv).wait() {
                    Ok(Either::A((r, sf))) =>
                        (Ok(r), Some(sf)),
                    Err(Either::A((re, sf))) =>
                        (Err(re), Some(sf)),
                    //we ignore server stream errors here, as they are always accompanied
                    // by main future's errors 'by design'
                    Ok(Either::B((_, rf))) | Err(Either::B((_, rf)))=>
                        (rf.wait(), None)
                },
            None => (rf.wait(), None)
        };
        self.s = s;
        r
    }

    ///exec and unwrap
    #[inline]
    #[cfg(test)]
    pub fn exec_u<R>(&mut self, rf: BFTT<R>) -> (bool, R) {
        biunwrap(self.exec_r(rf))
    }

    ///exec and extract errors
    #[inline]
    pub fn exec_x<R>(&mut self, rf: BFTT<R>) -> (Vec<IoError>, R) where R: ErrorExtractor {
        ErrorExtractor::from_result(self.exec_r(rf))
    }

    ///exec_y
    #[inline]
    pub fn exec_pair<C, R>(&mut self, rf: BFT<(C, R)>) -> (C, (Vec<IoError>, R)) where R: ErrorExtractor {
        match self.exec_r(rf) {
            Ok((c, r)) => (c, ErrorExtractor::extract_errors(r)),
            Err(()) => panic!("Unexpected future output on Error channel")
        }
    }


}

#[cfg(test)]
pub mod test {
    use super::*;

    pub struct TrivialAgeCheckerImpl;

    impl AgeChecker for TrivialAgeCheckerImpl {
        fn is_ok(&self, _t: Instant) -> bool {
            true
        }
    }

    pub struct TrivialAgeCheckerFactoryImpl;

    impl AgeCheckerFactory for TrivialAgeCheckerFactoryImpl {
        type E = TrivialAgeCheckerImpl;

        fn now() -> Instant {
            Instant::now()
        }

        fn checker(&self) -> TrivialAgeCheckerImpl {
            TrivialAgeCheckerImpl
        }
    }

    pub struct CI {
        pub n: usize
    }

    impl Connector<String, String> for CI {
        fn connect(&mut self, a: String) -> BFI<String> {
            self.n += 1;
            Box::new(ok(format!("CONN-{}-{}", (self.n - 1), a)))
        }
    }

    //pub type CPS = ConnectionPoolServer<String, String, String, CI, TrivialAgeCheckerFactoryImpl>;

    pub struct R { pub s: String }
    impl ErrorAccumulator for R {
        fn error(self, _e: IoError) -> Self {
            unimplemented!()
        }
    }
}


#[test]
fn test_cp_sync() {
    use self::test::*;

    let (c, s) =
        create_connection_pool::<String, String, String, CI, TrivialAgeCheckerFactoryImpl>(
            CI { n: 0 },
            TrivialAgeCheckerFactoryImpl,
            16,
            vec![]
        );

    let mut cr = Runner::new(s);

    let r = cr.exec_u(c.run(
        "A".to_owned(),
        Some("cA".to_owned()),
        R { s:"-x".to_owned() },
        |(c, r)| { let r = R { s: c.clone() + &r.s };  Box::new(ok((c, r))) }
        ));

    assert_eq!(r.1.s, "CONN-0-cA-x");

    let r = cr.exec_u(c.run(
            "A".to_owned(),
            Some("cA".to_owned()),
            R { s:"-y".to_owned() },
            |(c, r)| { let r = R { s: c.clone() + &r.s};  Box::new(ok((c, r))) }
        ));

    assert_eq!(r.1.s, "CONN-0-cA-y");

    let r = cr.exec_u(c.run(
            "B".to_owned(),
            Some("cB".to_owned()),
            R { s:"-z".to_owned() },
            |(c, r)| { let r = R { s: c.clone() + &r.s};  Box::new(ok((c, r))) }
        ));

    assert_eq!(r.1.s, "CONN-1-cB-z");
}

#[test]
fn test_cp_async() {
    use self::test::*;

    let (c, s) =
        create_connection_pool::<String, String, String, CI, TrivialAgeCheckerFactoryImpl>(
            CI { n: 0 },
            TrivialAgeCheckerFactoryImpl,
            16,
            vec![]
        );

    let _t = std::thread::spawn(move || {
        let _ = s.wait();
    });


    fn ts(w: BFTT<R>) -> R {
        match w.wait() {
            Ok(s) => s,
            _ => panic!("Invalid test outcome")
        }
    }

    let r = ts(
    c.run(
        "A".to_owned(),
        Some("cA".to_owned()),
        R { s:"-x".to_owned() },
        |(c, r)| { let r = R { s: c.clone() + &r.s};  Box::new(ok((c, r))) }
        )
    );

    assert_eq!(r.s, "CONN-0-cA-x");
}

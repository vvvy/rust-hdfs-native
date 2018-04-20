use std::net::{SocketAddr, ToSocketAddrs};
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
}

pub trait Connector<A, C> {
    fn connect(&mut self, a: A) -> BFI<C>;
}

#[derive(Debug)]
enum CPCmd<K, A, C> {
    Borrow(K, Option<A>, oneshot::Sender<C>),
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
                Box::new(f0.and_then(|c| {
                    let r = sk.send(c);
                    if r.is_err() { debug!("cpool.server sink send error (client gone?)") }
                    //we intentionally discard a connection (error outcome) here
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
            self.s.clone().send(CPCmd::Borrow(k, a, s))
                .map_err(|e| app_error!(other "ConnectionPoolClient: borrow: send error: {:?}", e).into())
                .and_then(|_|
                    r.map_err(|e|
                        app_error!(other "ConnectionPoolClient: borrow: send cancelled: {:?}", e).into()
                    )
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
    pub fn exec<R>(&mut self, rf: BFI<R>) -> IoResult<R> {
        use futures::future::Either;
        let mut s = None;
        std::mem::swap(&mut self.s, &mut s);
        let (r, s) = match s {
            Some(srv) =>
                match rf.select2(srv).wait() {
                    Ok(Either::A((r, sf))) => (Ok(r), sf),
                    Ok(Either::B((_s, _rf))) => return Err(app_error!(other "Premature end of server stream").into()),
                    Err(Either::A((re, sf))) => (Err(re), sf),
                    Err(Either::B((se, _rf))) => return Err(se)
                },
            None => return Err(app_error!(other "CP server dead").into()),
        };
        self.s = Some(s);
        r
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

    pub type CPS = ConnectionPoolServer<String, String, String, CI, TrivialAgeCheckerFactoryImpl>;
}


#[test]
fn test_cp_sync() {
    use self::test::*;
    use futures::future::Either;

    let (c, s) =
        create_connection_pool::<String, String, String, CI, TrivialAgeCheckerFactoryImpl>(
            CI { n: 0 },
            TrivialAgeCheckerFactoryImpl,
            16,
            vec![]
        );

    let mut cr = Runner::new(s);

    macro_rules! tw {
    ($r: expr) => { match $r {
        WaitResult::Ok(r, sf) => (r, sf),
        _ => panic!("Invalid test outcome")
    }};
    }

    let r = cr.exec(c.exec(
        "A".to_owned(),
        Some("cA".to_owned()),
        |c| { let r = c.clone() + "-x";  Box::new(ok((c, r))) }
        ));

    assert_eq!(r.unwrap(),"CONN-0-cA-x");

    let r = cr.exec(c.exec(
            "A".to_owned(),
            Some("cA".to_owned()),
            |c| { let r = c.clone() + "-y";  Box::new(ok((c, r))) }
        ));

    assert_eq!(r.unwrap(), "CONN-0-cA-y");

    let r = cr.exec(c.exec(
            "B".to_owned(),
            Some("cB".to_owned()),
            |c| { let r = c.clone() + "-z";  Box::new(ok((c, r))) }
        ));

    assert_eq!(r.unwrap(), "CONN-1-cB-z");
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


    fn ts(w: BFI<String>) -> String {
        match w.wait() {
            Ok(s) => s,
            _ => panic!("Invalid test outcome")
        }
    }

    let r = ts(
    c.exec(
        "A".to_owned(),
        Some("cA".to_owned()),
        |c| { let r = c.clone() + "-x";  Box::new(ok((c, r))) }
        )
    );

    assert_eq!(r, "CONN-0-cA-x");
}

//-------------------------------------------------------------------------------------------------

pub trait SyncConnectionPool {
    //fn run_nn<P>(&mut self, p: P) -> Box<Future<Item=P, Error=IoError>> where P: nn::ProtocolFsm;
    fn call_nn(&mut self, q: nn::NnQ) -> Result<nn::NnR>;
    fn run_nn<P>(&mut self, p: P) -> Result<P> where P: nn::ProtocolFsm + Send + 'static;
}

pub struct SyncConnectionPoolST {
    pooled: Option<nn::Connection>,
    nn_saddr: SocketAddr,
    eff_user: String
}

impl SyncConnectionPoolST {
    pub fn new(cfg: &config::Common) -> Result<SyncConnectionPoolST> {
        let addr = (&cfg.nn_host as &str, cfg.nn_port).to_socket_addrs()?.next().ok_or(app_error!(other "NN host not found"))?;
        Ok(SyncConnectionPoolST {
            pooled: None,
            nn_saddr: addr,
            eff_user: cfg.effective_user.clone()
        })
    }

    fn for_nn<F, R>(&mut self, f: F) -> Result<R> where
        F: FnOnce(nn::Connection) -> Box<Future<Item=(R, nn::Connection), Error=IoError>>
    {
        let mut c = None;

        std::mem::swap(&mut c, &mut self.pooled);

        let f0 = match c {
            Some(c) => Box::new(ok(c)),
            None => nn::Connection::connect(
                &self.nn_saddr,
                self.eff_user.clone()
            )
        };

        let fu =
            f0.and_then(|conn| f(conn));

        let (r, c) = fu.wait()?;
        self.pooled = Some(c);
        Ok(r)
    }
}

impl SyncConnectionPool for SyncConnectionPoolST {
    fn call_nn(&mut self, q: nn::NnQ) -> Result<nn::NnR> {
        self.for_nn(|c| c.call(q))
    }

    fn run_nn<P>(&mut self, p: P) -> Result<P> where P: nn::ProtocolFsm + Send + 'static {
        self.for_nn(|c| Box::new(c.run(p).map(|(c, p)| (p, c))))
    }
}


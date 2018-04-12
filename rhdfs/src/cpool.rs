use std::net::{SocketAddr, ToSocketAddrs};
use std::collections::{VecDeque, HashMap};
use std::collections::hash_map::Entry;
use std::hash::Hash;
use std::time::{Instant, Duration};
use std::marker::PhantomData;

use tokio_core::reactor::Core;
use futures::{Future, Stream, Sink};
use futures::future::ok;
use futures::sync::*;

use *;


/// Connection pool client abstraction. Must be easily cloneable. `K` is key type,
/// `A` is connection address, and `C` is connection type
pub trait ConnectionPool<K, A, C> {
    /// given a key, conn data, and a closure producing future of evaluation result `R` using
    /// connection 'C', return a future of that 'R'
    fn exec<F, R>(&self, key: K, addr: A, f: F) -> BFI<R> where
        R: 'static,
        F: FnOnce(C) -> BFI<(C, R)> + 'static;
}

pub trait Connector<A, C> {
    fn connect(&mut self, a: A) -> BFI<C>;
}

#[derive(Debug)]
enum CPCmd<K, A, C> {
    Borrow(K, A, oneshot::Sender<C>),
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
    fn new(max_age: Duration) -> AgeCheckerFactoryImpl {
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




/// CP impl that runs on a separate thread. `C` is connection, and `O` is connector type
#[derive(Debug)]
pub struct ConnectionPoolServer<K, A, C, O, T> where
    K: Eq + Hash
{
    m: HashMap<K, VecDeque<(C, Instant)>>,
    o: O,
    t: T,
    a_type: PhantomData<A>
}

impl<K, A, C, O, T> ConnectionPoolServer<K, A, C, O, T> where
    K: Eq + Hash + 'static,
    A: 'static,
    C: 'static,
    O: Connector<A, C> + 'static,
    T: AgeCheckerFactory + 'static
{
    pub fn new(o: O, t: T) -> ConnectionPoolServer<K, A, C, O, T> {
        ConnectionPoolServer {
            m: HashMap::new(),
            o,
            t,
            a_type: PhantomData
        }
    }

    fn pop(&mut self, k: K) -> Option<C> {
        match self.m.entry(k) {
            Entry::Vacant(..) => None,
            Entry::Occupied(mut e) => {
                loop {
                    let checker = self.t.checker();
                    let v = e.get_mut();
                    match v.pop_front() {
                        Some((c, t)) =>
                            if checker.is_ok(t) { break Some(c) }
                        None =>
                            break None
                    }
                }
            }
        }
    }

    fn push(&mut self, k: K, c: C) {
        self.m.entry(k).or_insert_with(|| VecDeque::new()).push_back((c, T::now()))
    }

    fn process_cmd(mut self, w: CPCmd<K, A, C>) -> BFI<Self> {
        match w {
            CPCmd::Borrow(k, a, sk) => {
                let f0: BFI<C> = match self.pop(k) {
                    Some(c) => Box::new(ok(c)),
                    None => self.o.connect(a)
                };
                Box::new(f0.and_then(|c| {
                    let r = sk.send(c);
                    if r.is_err() { trace!("cpool server: sk.send error") }
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
            .map_err(|_| app_error!(other "cpool: stream reception error").into())
            .fold(
                self,
                |cps, w| cps.process_cmd(w)
            )
        )
    }

    /// this differs from `into_future` in that it exposes the argument `r` in the loop,
    /// allowing for closing `r`
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
}
#[derive(Debug)]
pub struct ConnectionPoolServerEx<K, A, C, O, T> where
    K: Eq + Hash
{
    s: ConnectionPoolServer<K, A, C, O, T>,
    r: mpsc::Receiver<CPCmd<K, A, C>>
}

impl<K, A, C, O, T> ConnectionPoolServerEx<K, A, C, O, T> where
    K: Eq + Hash + 'static,
    A: 'static,
    C: 'static,
    O: Connector<A, C> + 'static,
    T: AgeCheckerFactory + 'static {
    pub fn into_future(self) -> BFI<ConnectionPoolServer<K, A, C, O, T>> {
        self.s.into_future(self.r)
    }
}


#[derive(Clone)]
pub struct ConnectionPoolClient<K, A, C> {
    s: mpsc::Sender<CPCmd<K, A, C>>
}

impl<K, A, C> ConnectionPoolClient<K, A, C> where
    K: 'static, A: 'static, C: 'static {
    fn borrow(&self, k: K, a: A) -> BFI<C> {
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
    K: Clone + 'static, A: 'static, C: 'static {

    fn exec<F, R>(&self, key: K, addr: A, f: F) -> BFI<R> where
        R: 'static,
        F: FnOnce(C) -> BFI<(C, R)> + 'static {

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

pub fn create_cpool<K, A, C, O, T>(o: O, t: T, buffer: usize)
                                   -> (ConnectionPoolClient<K, A, C>, ConnectionPoolServerEx<K, A, C, O, T>) where
    K: Eq + Hash + 'static,
    A: 'static,
    C: 'static,
    O: Connector<A, C> + 'static,
    T: AgeCheckerFactory  + 'static {
    let (s, r) = mpsc::channel(buffer);
    let c = ConnectionPoolClient { s };
    let srv =
        ConnectionPoolServerEx { s: ConnectionPoolServer::new(o, t), r };
    (c, srv)
}

#[cfg(test)]
mod test {
    use super::*;

    pub struct TrivialAgeCheckerImpl;

    impl AgeChecker for TrivialAgeCheckerImpl {
        fn is_ok(&self, t: Instant) -> bool {
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

    let (c, s0) =
        create_cpool::<String, String, String, CI, TrivialAgeCheckerFactoryImpl>(
            CI { n: 0 },
            TrivialAgeCheckerFactoryImpl,
            16
        );
    let s = s0.into_future();

    let mut core = Core::new().unwrap();

    fn ts(core: &mut Core, s: BFI<CPS>, w: BFI<String>) -> (BFI<CPS>, String) {
        match core.run(w.select2(s)) {
            Ok(Either::A((s, fcp))) => (fcp, s),
            _ => panic!("Invalid test outcome")
        }
    }

    /*
    match result {
        Ok(Either::A((s, fcp))) => println!("S is: {}", s), //S is: CONN-0-cd(A)-x
        Ok(Either::B((cp, fs))) => println!("S error"),
        Err(Either::A((se, fcp))) => println!("run error 1 {}", se),
        Err(Either::B((cpe, fs))) => println!("run error 2 {}", cpe),
    }*/

    let (s, r) = ts(
        &mut core, s,
        c.exec(
            "A".to_owned(),
            "cA".to_owned(),
            |c| { let r = c.clone() + "-x";  Box::new(ok((c, r))) }
        )
    );


    assert_eq!(r, "CONN-0-cA-x");

    let (s, r) = ts(
        &mut core, s,
        c.exec(
            "A".to_owned(),
            "cA".to_owned(),
            |c| { let r = c.clone() + "-y";  Box::new(ok((c, r))) }
        )
    );

    assert_eq!(r, "CONN-0-cA-y");

    let (s, r) = ts(
        &mut core, s,
        c.exec(
            "B".to_owned(),
            "cB".to_owned(),
            |c| { let r = c.clone() + "-z";  Box::new(ok((c, r))) }
        )
    );

    assert_eq!(r, "CONN-1-cB-z");
}
#[test]
fn test_cp_async() {
    use self::test::*;

    let (c, s) =
        create_cpool::<String, String, String, CI, TrivialAgeCheckerFactoryImpl>(
            CI { n: 0 },
            TrivialAgeCheckerFactoryImpl,
            16
        );

    let t = std::thread::spawn(move || {
        let _ = s.into_future().wait();
    });

    let mut core = Core::new().unwrap();

    fn ts(core: &mut Core, w: BFI<String>) -> String {
        match core.run(w) {
            Ok(s) => s,
            _ => panic!("Invalid test outcome")
        }
    }

    let r = ts(&mut core,
    c.exec(
        "A".to_owned(),
        "cA".to_owned(),
        |c| { let r = c.clone() + "-x";  Box::new(ok((c, r))) }
        )
    );

    assert_eq!(r, "CONN-0-cA-x");
}

//-------------------------------------------------------------------------------------------------

pub trait SyncConnectionPool {
    //fn run_nn<P>(&mut self, p: P) -> Box<Future<Item=P, Error=IoError>> where P: nn::ProtocolFsm;
    fn call_nn(&mut self, q: nn::NnQ) -> Result<nn::NnR>;
    fn run_nn<P>(&mut self, p: P) -> Result<P> where P: nn::ProtocolFsm + 'static;
}

pub struct SyncConnectionPoolST {
    pooled: Option<nn::Connection>,
    nn_saddr: SocketAddr,
    core: Core,
    eff_user: String
}

impl SyncConnectionPoolST {
    pub fn new(cfg: &config::Common) -> Result<SyncConnectionPoolST> {
        let addr = cfg.nn_hostport.to_socket_addrs()?.next().ok_or(app_error!(other "NN host not found"))?;
        Ok(SyncConnectionPoolST {
            pooled: None,
            nn_saddr: addr,
            core: Core::new()?,
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
                &self.core.handle(),
                &self.nn_saddr,
                self.eff_user.clone()
            )
        };

        let fu =
            f0.and_then(|conn| f(conn));

        let (r, c) = self.core.run(fu)?;
        self.pooled = Some(c);
        Ok(r)
    }
}

impl SyncConnectionPool for SyncConnectionPoolST {
    fn call_nn(&mut self, q: nn::NnQ) -> Result<nn::NnR> {
        self.for_nn(|c| c.call(q))
    }

    fn run_nn<P>(&mut self, p: P) -> Result<P> where P: nn::ProtocolFsm + 'static {
        self.for_nn(|c| Box::new(c.run(p).map(|(c, p)| (p, c))))
    }
}


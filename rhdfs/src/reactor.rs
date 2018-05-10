
use std::time::Duration;
use std::collections::HashMap;
use std::net::{SocketAddr};

use futures::{Future, Async};
use futures::future::{ok, err};

use nn;
use dt;
use cpool::*;
use *;

/// Connection key. For a DT, the string is a UUID of the DN. For a NN, the string is either `*`,
/// if no HA is configured, or NN's id otherwise
#[derive(Clone, PartialEq, Eq, Hash, Debug)]
pub enum CKey {
    NN(String),
    DT(String)
}

/// Connection address.
#[derive(Debug, Clone)]
pub enum CAddr {
    NN(SocketAddr),
    DT(SocketAddr)
}

/// Generic protocol FSM -- either namenode (type `FN`) or datatransfer (type `FD`).
/// `FN` and `FD` conform to `ProtocolFsm` of `nn::` and `dt::` respectively.
#[derive(Debug)]
pub enum CProtocolFsm<FN, FD> {
    NN(FN),
    DT(FD)
}

/// Generic reactor operation. The nature of `key` (NN or DT) is determined by the value of `p`.
#[derive(Debug)]
pub struct ReactorOperation<FN, FD> {
    pub key: String,
    pub addr: Option<CAddr>,
    pub p: CProtocolFsm<FN, FD>
}

impl<FN, FD> ReactorOperation<FN, FD> {
    pub fn for_nn(p: FN) -> ReactorOperation<FN, FD> {
        ReactorOperation { key: DEFAULT_NN_KEY.to_owned(), addr: None, p: CProtocolFsm::NN(p) }
    }
    pub fn for_dt(datanode_uuid: String, addr: SocketAddr, p: FD) -> ReactorOperation<FN, FD> {
        ReactorOperation {
            key: datanode_uuid,
            addr: Some(CAddr::DT(addr)),
            p: CProtocolFsm::DT(p)
        }
    }
}

/// Operations vector
pub type ReactorOpVec<FN, FD> = Vec<ReactorOperation<FN, FD>>;

//pub type RResult<V, R> = StdResult<(V, R), (IoError, R)>;

/// An FSM describing a 'meta operation' across a namenode and a set of datanodes
pub trait ReactorProtocolFsm where Self: Sized {
    /// Namenode operation type
    type FN;
    /// Datatransfer operation type
    type FD;
    fn start(self) -> (ReactorOpVec<Self::FN, Self::FD>, Self);
    fn complete(self, op: ReactorOperation<Self::FN, Self::FD>) -> (ReactorOpVec<Self::FN, Self::FD>, Self);
    fn error(self, op: ReactorOperation<Self::FN, Self::FD>) -> (ReactorOpVec<Self::FN, Self::FD>, Self);
}

/// Future of `ReactorOperation`
type ReactorOpF<FN, FD> = BFTT<ReactorOperation<FN, FD>>;

pub enum ReactorConnection {
    NN(nn::Connection),
    DT(dt::Connection)
}

pub struct ReactorClient {
    pool: ConnectionPoolClient<CKey, CAddr, ReactorConnection>
}

impl Clone for ReactorClient {
    fn clone(&self) -> Self {
        ReactorClient { pool: self.pool.clone() }
    }
}

/// Misc session data
pub struct SessionData {
    ///(NN) effective user in IpcConnectionContextProto.UserInformationProto
    pub effective_user: String,
}

pub struct ConnectorImpl {
    nat: HashMap<SocketAddr, SocketAddr>,
    session_data: SessionData,
    connector_id: u64,
    connection_n: usize
}

impl ConnectorImpl {
    fn new(session_data: SessionData, init_nat: Vec<(SocketAddr, SocketAddr)>) -> ConnectorImpl {
        let connector_id = rand::random();
        let mut nat = HashMap::new();
        for (k, v) in init_nat { nat.insert(k, v); }
        ConnectorImpl { nat, session_data, connector_id, connection_n: 0 }
    }
    fn next_client_name(&mut self) -> String {
        //org.apache.hadoop.hdfs.DFSClient.DFSClient:
        //this.clientName = "DFSClient_" + dfsClientConf.taskId + "_" +
        //    DFSUtil.getRandom().nextInt()  + "_" + Thread.currentThread().getId();

        let rv = format!("RDFSClient_{}_{}", self.connector_id, self.connection_n);
        self.connection_n += 1;
        rv
    }

    #[inline]
    fn translate<'a>(&'a self, a: &'a SocketAddr) -> &'a SocketAddr {
        self.nat.get(a).unwrap_or(a)
    }
}

impl Connector<CAddr, ReactorConnection> for ConnectorImpl {
    fn connect(&mut self, a: CAddr) -> BFI<ReactorConnection> {
        match a {
            CAddr::NN(addr) =>
                Box::new(nn::Connection::connect(
                    self.translate(&addr),
                    self.session_data.effective_user.clone()
                ).map(|c| ReactorConnection::NN(c))),
            CAddr::DT(addr) => {
                let cname = self.next_client_name();
                Box::new(dt::Connection::connect(
                    self.translate(&addr),
                    cname,
                ).map(|c| ReactorConnection::DT(c)))
            }
        }
    }
}

pub type ReactorServer = ConnectionPoolServer<
    CKey, CAddr, ReactorConnection,
    ConnectorImpl, AgeCheckerFactoryImpl
>;

/// Creates reactor client/server pair. Clients may be cloned and sent between threads.
/// Arguments are server receiver buffer size and maximum connection age. The maximum connection
/// age corresponds to `ipc.client.connection.maxidletime` Hadoop setting (defaults to 10s).
/// This should not exceed the actual value configured in the cluster.
pub fn create_reactor(
    buffer_size: usize,
    max_age: Duration,
    init_a: Vec<(CKey, CAddr)>,
    init_nat: Vec<(SocketAddr, SocketAddr)>,
    session_data: SessionData) -> (ReactorClient, BFI<ReactorServer>) {
    let (c, s) =
        create_connection_pool(
            ConnectorImpl::new(session_data ,init_nat),
            AgeCheckerFactoryImpl::new(max_age),
            buffer_size,
            init_a);
    (ReactorClient { pool: c }, s)
}

pub const DEFAULT_NN_KEY: &'static str = "*";

pub fn default_nn_key() -> CKey { CKey::NN(DEFAULT_NN_KEY.to_owned()) }

impl ReactorClient {
    #[inline]
    pub fn run<P>(self, p: P)-> BFT<(Self, P)>  where
        P: ReactorProtocolFsm + Send + 'static,
        P::FN: nn::ProtocolFsm + Send + 'static,
        P::FD: dt::ProtocolFsm + Send + 'static {
        Box::new(FsmRunner::new(self, p))
    }

    fn conn_type_error() -> IoError {
        app_error!(other "INTERNAL: conn type mismatch").into()
    }

    pub fn run_nn_ex<FN>(&self, k: String, a: Option<CAddr>, p: FN) -> BFTT<FN> where
        FN: nn::ProtocolFsm + Send + 'static {
        self.pool.run(
            CKey::NN(k),
            a,
            p,
            |(c, p)|
                if let ReactorConnection::NN(c) = c {
                    Box::new(c.run(p).map(|(c, p)| (ReactorConnection::NN(c), p)))
                } else {
                    Box::new(err(p.error(Self::conn_type_error())))
                }
        )
    }

    pub fn run_nn<FN>(&self, p: FN) -> BFTT<FN> where
        FN: nn::ProtocolFsm + Send + 'static {
        self.run_nn_ex(DEFAULT_NN_KEY.to_owned(), None, p)
    }

    pub fn call_nn_ex(&self, k: String, a: Option<CAddr>, q: nn::NnQ) -> BFI<nn::NnR> {
        self.pool.exec(
            CKey::NN(k),
            a,
            |c|
                if let ReactorConnection::NN(c) = c {
                    Box::new(c.call(q).map(|(r, c)| (ReactorConnection::NN(c), r)))
                } else {
                    Box::new(err(Self::conn_type_error()))
            }
        )
    }

    pub fn call_nn(&self, q: nn::NnQ) -> BFI<nn::NnR> {
        self.call_nn_ex(DEFAULT_NN_KEY.to_owned(), None, q)
    }

    pub fn run_dt<FD>(&self, k: String, a: Option<CAddr>, p: FD) -> BFTT<FD> where
        FD: dt::ProtocolFsm + Send + 'static {
        self.pool.run(
            CKey::DT(k),
            a,
            p,
            |(c, p)|
                if let ReactorConnection::DT(c) = c {
                    Box::new(c.run(p).map(|(c, p)| (ReactorConnection::DT(c), p)))
                } else {
                    Box::new(err(p.error(Self::conn_type_error())))
                }
        )
    }

    /// execute a generic operation against the connection pools
    fn exec_op<FN, FD>(&self, op: ReactorOperation<FN, FD>) -> BFTT<ReactorOperation<FN, FD>> where
        FN: nn::ProtocolFsm + Send + 'static,
        FD: dt::ProtocolFsm + Send + 'static {
        match op {
            ReactorOperation { key: k, addr: a, p: CProtocolFsm::NN(p) } => {
                let (k1, a1) = (k.clone(), a.clone());
                bimap(
                    self.run_nn_ex(k, a, p),
                    |p1| ReactorOperation { key: k1, addr: a1, p: CProtocolFsm::NN(p1) }
                )
            },
            ReactorOperation { key: k, addr: a, p: CProtocolFsm::DT(p) } => {
                let (k1, a1) = (k.clone(), a.clone());
                bimap(
                    self.run_dt(k, a, p),
                    |p1| ReactorOperation { key: k1, addr: a1, p: CProtocolFsm::DT(p1) }
                )
            }
        }
    }
}

type SelectAllBF<FN, FD> =  BF<
    (ReactorOperation<FN, FD>, usize, Vec<ReactorOpF<FN, FD>>),
    (ReactorOperation<FN, FD>, usize, Vec<ReactorOpF<FN, FD>>)
>;



enum FsmRunner<P> where P: ReactorProtocolFsm + Send + 'static {
    Init(ReactorClient, P),
    Many(ReactorClient, P, SelectAllBF<P::FN, P::FD>),
    Null
}

type FRR<P> = StdResult<Async<(ReactorClient, P)>, ()>;

impl<P> Future for FsmRunner<P> where
    P: ReactorProtocolFsm + Send + 'static,
    P::FN: nn::ProtocolFsm + Send + 'static,
    P::FD: dt::ProtocolFsm + Send + 'static {
    type Item = (ReactorClient, P);
    type Error = ();

    fn poll(&mut self) -> FRR<P> {
        #[inline]
        fn absolutely_not_ready<P>(s: FsmRunner<P>) -> (FsmRunner<P>, Option<FRR<P>>) where
            P: ReactorProtocolFsm + Send + 'static { (s, None)  }

        #[inline]
        fn not_ready<P>(s: FsmRunner<P>) -> (FsmRunner<P>, Option<FRR<P>>) where
            P: ReactorProtocolFsm + Send + 'static { (s, Some(Ok(Async::NotReady)))  }

        #[inline]
        fn ready<P>(v: FRR<P>) -> (FsmRunner<P>, Option<FRR<P>>) where
            P: ReactorProtocolFsm + Send + 'static { (FsmRunner::Null, Some(v)) }

        fn process_result<P>(c: ReactorClient, (r, p): (ReactorOpVec<P::FN, P::FD>, P), rest: Vec<ReactorOpF<P::FN, P::FD>>) -> (FsmRunner<P>, Option<FRR<P>>) where
            P: ReactorProtocolFsm + Send + 'static,
            P::FN: nn::ProtocolFsm + Send + 'static,
            P::FD: dt::ProtocolFsm + Send + 'static {
            let ops = vec_plus(rest, r.into_iter().map(|op| c.exec_op(op)).collect());
            if ops.is_empty() {
                ready(Ok(Async::Ready((c, p))))
            } else {
                //TODO: specially handle ops.len() == 1 via `FsmRunner::One(ReactorClient, P, ReactorOpF<FN, FD>)`
                absolutely_not_ready(FsmRunner::Many(c,p, Box::new(futures::future::select_all(ops))))
            }
        }

        loop {
            let (s1, rv) = match std::mem::replace(self, FsmRunner::Null) {
                FsmRunner::Init(c, p) => process_result(c, p.start(), vec![]),
                FsmRunner::Many(c, p, mut f) => match f.poll() {
                    Ok(Async::NotReady) =>
                        not_ready(FsmRunner::Many(c, p, f)),
                    Ok(Async::Ready((completed, _offset, rest))) =>
                        process_result(c, p.complete(completed), rest),
                    Err((in_error, _offset, rest)) =>
                        process_result(c, p.error(in_error), rest)
                }
                FsmRunner::Null =>
                    panic!("Unfused call to completed FsmRunner")
            };
            *self = s1;
            if let Some(v) = rv { break v }
        }
    }
}

impl<P> FsmRunner<P> where
    P: ReactorProtocolFsm + Send + 'static,
    P::FN: nn::ProtocolFsm + Send + 'static,
    P::FD: dt::ProtocolFsm + Send + 'static {

    fn new(c: ReactorClient, p: P) -> FsmRunner<P> { FsmRunner::Init(c, p) }

}


pub type ReactorRunner = Runner<ReactorServer>;


/*
#[cfg(test)]
pub mod test {
    use super::*;
    use cpool::test::*;
}

#[test]
fn test_reactor() {

}
*/
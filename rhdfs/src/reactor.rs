
use std::time::Duration;
use futures::{Future};
use futures::future::{ok, err};

use nn;
use dt;
use cpool::*;
use *;
use std::net::{SocketAddr};

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

type BSF<P: ReactorProtocolFsm> = BFTT<((ReactorOperation<P::FN, P::FD>, usize, Vec<ReactorOpF<P::FN, P::FD>>), P)>;

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
    session_data: SessionData,
    connector_id: u64,
    connection_n: usize
}

impl ConnectorImpl {
    fn new(session_data: SessionData) -> ConnectorImpl {
        let connector_id = rand::random();
        ConnectorImpl { session_data, connector_id, connection_n: 0 }
    }
    fn next_client_name(&mut self) -> String {
        //org.apache.hadoop.hdfs.DFSClient.DFSClient:
        //this.clientName = "DFSClient_" + dfsClientConf.taskId + "_" +
        //    DFSUtil.getRandom().nextInt()  + "_" + Thread.currentThread().getId();

        let rv = format!("RDFSClient_{}_{}", self.connector_id, self.connection_n);
        self.connection_n += 1;
        rv
    }
}


impl Connector<CAddr, ReactorConnection> for ConnectorImpl {
    fn connect(&mut self, a: CAddr) -> BFI<ReactorConnection> {
        match a {
            CAddr::NN(addr) =>
                Box::new(nn::Connection::connect(&addr, self.session_data.effective_user.clone()).map(|c| ReactorConnection::NN(c))),
            CAddr::DT(addr) =>
                Box::new(dt::Connection::connect(&addr, self.next_client_name()).map(|c| ReactorConnection::DT(c)))
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
pub fn create_reactor(buffer_size: usize, max_age: Duration, init_a: Vec<(CKey, CAddr)>, session_data: SessionData) -> (ReactorClient, BFI<ReactorServer>) {
    let (c, s) =
        create_connection_pool(ConnectorImpl::new(session_data), AgeCheckerFactoryImpl::new(max_age), buffer_size, init_a);
    (ReactorClient { pool: c }, s)
}

pub const DEFAULT_NN_KEY: &'static str = "*";

pub fn default_nn_key() -> CKey { CKey::NN(DEFAULT_NN_KEY.to_owned()) }

impl ReactorClient {
    #[inline]
    pub fn run<P>(self, p: P)-> BFTT<P>  where
        P: ReactorProtocolFsm + Send + 'static,
        P::FN: nn::ProtocolFsm + Send + 'static,
        P::FD: dt::ProtocolFsm + Send + 'static {
        self.fsm_start(p)
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

    fn fsm_start<P>(self, p: P)-> BFTT<P>  where
        P: ReactorProtocolFsm + Send + 'static,
        P::FN: nn::ProtocolFsm + Send + 'static,
        P::FD: dt::ProtocolFsm + Send + 'static {
        let (y, p) = p.start(); //match p.start() { Ok(v) => v, Err(e) => return Box::new(err(e)) };
        let ops = y.into_iter().map(|op| self.exec_op(op)).collect();
        self.fsm_launch(p, ops)
    }

    /// Recurring `ReactorProtocolFsm` evaluation step
    fn fsm_step<P>(self, f: BSF<P>) -> BFTT<P> where
        P: ReactorProtocolFsm + Send + 'static,
        P::FN: nn::ProtocolFsm + Send + 'static,
        P::FD: dt::ProtocolFsm + Send + 'static {

        Box::new(f.then(|w| {
            let ((y, p), mut ops) = match w {
                Ok(((r, _, ops), p)) => (p.complete(r), ops),
                Err(((r, _, ops), p)) => (p.error(r), ops)
            };
            let mut z = y.into_iter().map(|op| self.exec_op(op)).collect();
            ops.append(&mut z);
            self.fsm_launch(p, ops)
        }))
    }

    fn fsm_launch<P>(self, p: P, ops: Vec<ReactorOpF<P::FN, P::FD>>) -> BFTT<P> where
        P: ReactorProtocolFsm + Send + 'static,
        P::FN: nn::ProtocolFsm + Send + 'static,
        P::FD: dt::ProtocolFsm + Send + 'static {
        if ops.is_empty() {
            Box::new(ok(p))
        } else {
            let bf = bimap(
                futures::future::select_all(ops),
                |rix| (rix, p)
            );
            self.fsm_step(bf)
        }
    }
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
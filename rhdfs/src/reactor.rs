
use std::time::Duration;
use futures::{Future};
use futures::future::{ok, err};

use nn;
use dt;
use cpool::*;
use *;
use std::net::{SocketAddr, ToSocketAddrs};

/// Connection key. For a DT, the string is a UUID of the DN. For a NN, the string is either `*`,
/// if no HA is configured, or NN's id otherwise
#[derive(Clone, PartialEq, Eq, Hash, Debug)]
pub enum CKey {
    NN(String),
    DT(String)
}

/// Connection address.
#[derive(Clone)]
pub enum CAddr {
    NN { host: String, port: u16, eff_user: String },
    DT { host: String, port: u16 }
}

/// Generic protocol FSM -- either namenode (type `FN`) or datatransfer (type `FD`).
/// `FN` and `FD` conform to `ProtocolFsm` of `nn::` and `dt::` respectively.
pub enum CProtocolFsm<FN, FD> {
    NN(FN),
    DT(FD)
}

/// Generic reactor operation. The nature of `key` (NN or DT) is determined by the value of `p`.
pub struct ReactorOperation<FN, FD> {
    key: String,
    addr: Option<CAddr>,
    p: CProtocolFsm<FN, FD>
}

/// Operations vector
pub type ReactorOpVec<FN, FD> = Vec<ReactorOperation<FN, FD>>;

/// An FSM describing a 'meta operation' across a namenode and a set of datanodes
pub trait ReactorProtocolFsm {
    /// Namenode operation type
    type FN;
    /// Datatransfer operation type
    type FD;
    fn start(self) -> (ReactorOpVec<Self::FN, Self::FD>, Self);
    fn complete(self, ops: ReactorOperation<Self::FN, Self::FD>) -> (ReactorOpVec<Self::FN, Self::FD>, Self);
    //TODO proper error handling
    //fn error(self, e: IoError) -> (ReactorOpVec<Self::FN, Self::FD>, Self);
}

/// Future of `ReactorOperation`
type ReactorOpF<FN, FD> = BFI<ReactorOperation<FN, FD>>;

type BSF<P: ReactorProtocolFsm> = BFI<((ReactorOperation<P::FN, P::FD>, usize, Vec<ReactorOpF<P::FN, P::FD>>), P)>;

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



/// Resolves the hostname to its first Ip
fn resolve_one<'a>(host: String, port: u16, role: &'a str) -> IoResult<SocketAddr> {
    match ((&host as &str, port)).to_socket_addrs()?.next() {
        Some(a) => Ok(a),
        None => Err(app_error!(other "Could not resolve `{}` host {}", role, host).into())
    }
}

pub struct ConnectorImpl;

const CLIENT_NAME: &str = "rhdfs";

impl Connector<CAddr, ReactorConnection> for ConnectorImpl {
    //TODO move resolution away from CP server thread (?)
    fn connect(&mut self, a: CAddr) -> BFI<ReactorConnection> {
        match a {
            CAddr::NN { host, port, eff_user } =>
                match resolve_one(host, port, "namenode") {
                    Ok(a) => Box::new(nn::Connection::connect(&a, eff_user).map(|c| ReactorConnection::NN(c))),
                    Err(e) => Box::new(err(e))
                }
            CAddr::DT { host, port } =>
                match resolve_one(host, port, "datanode") {
                    Ok(a) => Box::new(dt::Connection::connect(&a, CLIENT_NAME.to_owned()).map(|c| ReactorConnection::DT(c))),
                    Err(e) => Box::new(err(e))
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
pub fn create_reactor(buffer_size: usize, max_age: Duration, init_a: Vec<(CKey, CAddr)>) -> (ReactorClient, BFI<ReactorServer>) {
    let (c, s) =
        create_connection_pool(ConnectorImpl, AgeCheckerFactoryImpl::new(max_age), buffer_size, init_a);
    (ReactorClient { pool: c }, s)
}

pub const DEFAULT_NN_KEY: &'static str = "*";

pub fn default_nn_key() -> CKey { CKey::NN(DEFAULT_NN_KEY.to_owned()) }

impl ReactorClient {
    #[inline]
    pub fn run<P>(self, p: P)-> BFI<P>  where
        P: ReactorProtocolFsm + Send + 'static,
        P::FN: nn::ProtocolFsm + Send + 'static,
        P::FD: dt::ProtocolFsm + Send + 'static {
        self.fsm_start(p)
    }

    pub fn run_nn_ex<FN>(&self, k: String, a: Option<CAddr>, p: FN) -> BFI<FN> where
        FN: nn::ProtocolFsm + Send + 'static {
        self.pool.exec(
            CKey::NN(k),
            a,
            |c| match c {
                ReactorConnection::NN(c) =>
                    Box::new(c.run(p).map(|(c, p)| (ReactorConnection::NN(c), p))),
                ReactorConnection::DT(_) =>
                    Box::new(err(app_error!(other "INTERNAL: conn type mismatch").into())),
            }
        )
    }

    pub fn run_nn<FN>(&self, p: FN) -> BFI<FN> where
        FN: nn::ProtocolFsm + Send + 'static {
        self.run_nn_ex(DEFAULT_NN_KEY.to_owned(), None, p)
    }

    pub fn call_nn_ex(&self, k: String, a: Option<CAddr>, q: nn::NnQ) -> BFI<nn::NnR> {
        self.pool.exec(
            CKey::NN(k),
            a,
            |c| match c {
                ReactorConnection::NN(c) =>
                    Box::new(c.call(q).map(|(r, c)| (ReactorConnection::NN(c), r))),
                ReactorConnection::DT(_) =>
                    Box::new(err(app_error!(other "INTERNAL: conn type mismatch").into())),
            }
        )
    }

    pub fn call_nn(&self, q: nn::NnQ) -> BFI<nn::NnR> {
        self.call_nn_ex(DEFAULT_NN_KEY.to_owned(), None, q)
    }

    pub fn run_dt<FD>(&self, k: String, a: Option<CAddr>, p: FD) -> BFI<FD> where
        FD: dt::ProtocolFsm + Send + 'static {
        self.pool.exec(
            CKey::DT(k),
            a,
            |c| match c {
                ReactorConnection::NN(_) =>
                    Box::new(err(app_error!(other "INTERNAL: conn type mismatch").into())),
                ReactorConnection::DT(c) =>
                    Box::new(c.run(p).map(|(c, p)| (ReactorConnection::DT(c), p))),
            }
        )
    }

    /// execute a generic operation against the connection pools
    fn exec_op<FN, FD>(&self, op: ReactorOperation<FN, FD>) -> BFI<ReactorOperation<FN, FD>> where
        FN: nn::ProtocolFsm + Send + 'static,
        FD: dt::ProtocolFsm + Send + 'static {
        match op {
            ReactorOperation { key: k, addr: a, p: CProtocolFsm::NN(p) } => {
                let (k1, a1) = (k.clone(), a.clone());
                Box::new(
                    self.run_nn_ex(k, a, p)
                        .map(|p1| ReactorOperation { key: k1, addr: a1, p: CProtocolFsm::NN(p1) })
                )
            },
            ReactorOperation { key: k, addr: a, p: CProtocolFsm::DT(p) } => {
                let (k1, a1) = (k.clone(), a.clone());
                Box::new(
                    self.run_dt(k, a, p)
                        .map(|p1| ReactorOperation { key: k1, addr: a1, p: CProtocolFsm::DT(p1) })
                )
            }
        }
    }

    fn fsm_start<P>(self, p: P)-> BFI<P>  where
        P: ReactorProtocolFsm + Send + 'static,
        P::FN: nn::ProtocolFsm + Send + 'static,
        P::FD: dt::ProtocolFsm + Send + 'static {

        let (y, p) = p.start();
        let ops = y.into_iter().map(|op| self.exec_op(op)).collect();
        self.fsm_launch(p, ops)
    }

    /// Recurring `ReactorProtocolFsm` evaluation step
    fn fsm_step<P>(self, f: BSF<P>) -> BFI<P> where
        P: ReactorProtocolFsm + Send + 'static,
        P::FN: nn::ProtocolFsm + Send + 'static,
        P::FD: dt::ProtocolFsm + Send + 'static {
        Box::new(f.and_then(|((r, _i, mut ops), p)| {
            let (y, p) = p.complete(r);
            let mut z = y.into_iter().map(|op| self.exec_op(op)).collect();
            ops.append(&mut z);
            self.fsm_launch(p, ops)
        }))
    }

    fn fsm_launch<P>(self, p: P, ops: Vec<ReactorOpF<P::FN, P::FD>>) -> BFI<P> where
        P: ReactorProtocolFsm + Send + 'static,
        P::FN: nn::ProtocolFsm + Send + 'static,
        P::FD: dt::ProtocolFsm + Send + 'static {
        if ops.is_empty() {
            Box::new(ok(p))
        } else {
            let bf = Box::new(
                futures::future::select_all(ops)
                    //TODO get rid of this (provide proper error handling)
                    .map_err(|(e, _i, _x)| e)
                    .map(|rix| (rix, p))
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
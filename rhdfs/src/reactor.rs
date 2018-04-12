
use futures::{Future};
use futures::future::{ok};

use nn;
use dt;
use cpool::*;
use *;

/// Connection key, a datanode uuid, or namenode ???
pub type CKey = String;

/// Connection address.
#[derive(Clone)]
pub struct CAddr {
    hostname: String,
    port: u16
}

/// Generic reactor operation -- either namenode (type `N`) or datatransfer (type `D`) operation
/// `FN` and `FD` conform to `ProtocolFsm` of `nn::` and `dt::` respectively.
pub enum ReactorOperation<FN, FD> {
    NN(CKey, CAddr, FN),
    DT(CKey, CAddr, FD)
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

#[derive(Clone)]
pub struct ReactorContext<PN, PD> {
    pool_n: PN,
    pool_d: PD
}

impl<PN, PD> ReactorContext<PN, PD> where
    PN: ConnectionPool<CKey, CAddr, nn::Connection> + 'static,
    PD: ConnectionPool<CKey, CAddr, dt::Connection> + 'static {

    #[inline]
    pub fn run<P>(self, p: P)-> BFI<P>  where
        P: ReactorProtocolFsm + 'static,
        P::FN: nn::ProtocolFsm + 'static,
        P::FD: dt::ProtocolFsm + 'static {
        self.fsm_start(p)
    }

    /// execute a generic operation against the connection pools
    fn exec_op<FN, FD>(&self, op: ReactorOperation<FN, FD>) -> BFI<ReactorOperation<FN, FD>> where
        FN: nn::ProtocolFsm + 'static,
        FD: dt::ProtocolFsm + 'static {
        match op {
            ReactorOperation::NN(k, a, n) => {
                let (k1, a1) = (k.clone(), a.clone());
                Box::new(self.pool_n.exec(
                    k, a, |c| Box::new(c.run(n))
                ).map(|n1| ReactorOperation::NN(k1, a1, n1)))
            },
            ReactorOperation::DT(k, a, n) => {
                let (k1, a1) = (k.clone(), a.clone());
                Box::new(self.pool_d.exec(
                    k, a, |c| Box::new(c.run(n))
                ).map(|n1| ReactorOperation::DT(k1, a1, n1)))
            }
        }
    }

    fn fsm_start<P>(self, p: P)-> BFI<P>  where
        P: ReactorProtocolFsm + 'static,
        P::FN: nn::ProtocolFsm + 'static,
        P::FD: dt::ProtocolFsm + 'static {

        let (y, p) = p.start();
        let ops = y.into_iter().map(|op| self.exec_op(op)).collect();
        self.fsm_launch(p, ops)
    }

    /// Recurring `ReactorProtocolFsm` evaluation step
    fn fsm_step<P>(self, f: BSF<P>) -> BFI<P> where
        P: ReactorProtocolFsm + 'static,
        P::FN: nn::ProtocolFsm + 'static,
        P::FD: dt::ProtocolFsm + 'static {
        Box::new(f.and_then(|((r, _i, mut ops), p)| {
            let (y, p) = p.complete(r);
            let mut z = y.into_iter().map(|op| self.exec_op(op)).collect();
            ops.append(&mut z);
            self.fsm_launch(p, ops)
        }))
    }

    fn fsm_launch<P>(self, p: P, ops: Vec<ReactorOpF<P::FN, P::FD>>) -> BFI<P> where
        P: ReactorProtocolFsm + 'static,
        P::FN: nn::ProtocolFsm + 'static,
        P::FD: dt::ProtocolFsm + 'static {
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

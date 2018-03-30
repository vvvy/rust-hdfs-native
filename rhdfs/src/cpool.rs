use std::net::{SocketAddr, ToSocketAddrs};
use tokio_core::reactor::Core;
use futures::{Future};
use futures::future::ok;

use protobuf_api::*;
use nn;
use dt;
use *;


pub trait ConnectionPool {
    //fn run_nn<P>(&mut self, p: P) -> Box<Future<Item=P, Error=IoError>> where P: nn::ProtocolFsm;
    fn run_nn<P>(&mut self, p: P) -> Result<P> where P: nn::ProtocolFsm + 'static;
}

pub struct ConnectionPoolST {
    pooled: Option<nn::Connection>,
    nn_saddr: SocketAddr,
    core: Core,
    eff_user: String
}

impl ConnectionPoolST {
    pub fn new(cfg: &config::Common) -> Result<ConnectionPoolST> {
        let addr = cfg.nn_hostport.to_socket_addrs()?.next().ok_or(app_error!(other "NN host not found"))?;
        Ok(ConnectionPoolST {
            pooled: None,
            nn_saddr: addr,
            core: Core::new()?,
            eff_user: cfg.effective_user.clone()
        })
    }
}

impl ConnectionPool for ConnectionPoolST {
    fn run_nn<P>(&mut self, p: P) -> Result<P> where P: nn::ProtocolFsm + 'static {
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

        let f =
            f0.and_then(|conn| conn.run(p));

        let (c, p) = self.core.run(f)?;
        self.pooled = Some(c);
        Ok(p)
    }
}


pub(crate) mod read_listing;

use proto_tools::*;
use nn;
use *;
use std::net::SocketAddr;
use futures::prelude::*;

#[derive(Clone, Debug)]
pub struct NNCD {
    pub addr: SocketAddr,
    pub eff_user: String,
    pub client_id: Option<Vec<u8>>
}

impl Into<BFI<nn::Connection>> for NNCD {
    fn into(self) -> BFI<nn::Connection> {
        match self.client_id {
            Some(client_id) => nn::Connection::connect_det(&self.addr, client_id, self.eff_user),
            None => nn::Connection::connect(&self.addr, self.eff_user),
        }
    }
}

pub type NNCQ = CQ<nn::NnQ, NNCD>;
//pub type NNChannel = AutoChannel<nn::NnQ, nn::NnR, nn::Connection, NNCD>;

pub struct NNChannel {
    inner: AutoChannel<nn::NnQ, nn::NnR, nn::Connection, NNCD>
}
impl NNChannel {
    pub fn new(cdata: NNCD, timeout: Duration) -> NNChannel {
        NNChannel { inner: AutoChannel::new(cdata, timeout) }
    }
}
impl Stream for NNChannel {
    type Item = nn::NnR;
    type Error = Error;

    fn poll(&mut self) -> Result<Async<Option<nn::NnR>>> {
        self.inner.poll()
    }
}

impl Sink for NNChannel {
    type SinkItem = nn::NnQ;
    type SinkError = Error;

    fn start_send(&mut self, item: nn::NnQ) -> Result<AsyncSink<nn::NnQ>> {
        self.inner.start_send(item)
    }

    fn poll_complete(&mut self) -> Result<Async<()>> {
        self.inner.poll_complete()
    }

    fn close(&mut self) -> Result<Async<()>> {
        self.inner.close()
    }
}


pub use self::read_listing::GetListing;
pub fn get_listing(c: NNChannel, source: String, need_location: bool) -> GetListing {
    GetListing::new(c, source, need_location)
}
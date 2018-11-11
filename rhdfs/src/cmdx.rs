
use std::net::SocketAddr;
use std::collections::{VecDeque, HashMap};
use futures::{Stream, Async, Sink, AsyncSink};

use dt::*;
use nn::*;
use *;

use util::*;
use proto_tools::{ProtoFrontEndSourceQ, UserDeliver};

/// Connector abstraction
pub trait Connector<C, A> {
    /// Takes a reference to an address `a` and returns boxed future of the connection `C`
    fn connect(&mut self, a: &A) -> BFI<C>;
}

/// Misc session data
pub struct SessionData {
    ///(NN) effective user in IpcConnectionContextProto.UserInformationProto
    pub effective_user: String,
    ///(NN) force client id (for debugging purposes). Auto-generated if None.
    pub forced_client_id: Option<Vec<u8>>
}

/// A connector that supports static NAT
pub struct NatConnector {
    nat: HashMap<SocketAddr, SocketAddr>,
    session_data: SessionData,
    connector_id: u64,
    connection_n: usize
}

impl NatConnector {
    fn new(session_data: SessionData, init_nat: Vec<(SocketAddr, SocketAddr)>) -> NatConnector {
        let connector_id = rand::random();
        let mut nat = HashMap::new();
        for (k, v) in init_nat { nat.insert(k, v); }
        NatConnector { nat, session_data, connector_id, connection_n: 0 }
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

impl Connector<nn::Connection_, SocketAddr> for NatConnector {
    fn connect(&mut self, a: &SocketAddr) -> BFI<nn::Connection_> {
        match &self.session_data.forced_client_id {
            None =>
                nn::Connection_::connect(
                    self.translate(a),
                    self.session_data.effective_user.clone()
                ),
            Some(cn) =>
                nn::Connection_::connect_det(
                    self.translate(a),
                    cn.clone(),
                    self.session_data.effective_user.clone()
                )
        }

    }
}

impl Connector<dt::Connection_, SocketAddr> for NatConnector {
    fn connect(&mut self, a: &SocketAddr) -> BFI<dt::Connection_> {
        let cname = self.next_client_name();
        Box::new(dt::Connection_::connect(
            self.translate(a),
            cname,
        ))
    }
}

/// Connection manager state
enum CMS<C> {
    None,
    Connecting(BFI<C>),
    Active(C)
}


//TODO add idle timer functionality
///Connection manager. Does connection management for an upper layer together with passing
///messages up/down
struct CM<C, Q, A> {
    s: CMS<C>,
    q: VecDeque<Q>,
    a: Option<A>
}

const OUT_Q_SIZE: usize = 16;

impl<C, Q, R, A> CM<C, Q, A> where
    C: Sink<SinkItem=Q, SinkError=Error> + Stream<Item=R, Error=Error> {

    fn new() -> CM<C, Q, A> {
        CM {
            s: CMS::None,
            q: VecDeque::new(),
            a: None
        }
    }

    fn push_q(&mut self, (req, a): (Q, Option<A>)) -> AsyncSink<(Q, Option<A>)> {
        if self.q.len() >= OUT_Q_SIZE {
            AsyncSink::NotReady((req, a))
        } else {
            if a.is_some() { self.a = a }
            self.q.push_back(req);
            AsyncSink::Ready
        }
    }

    fn run<O>(&mut self, c: &mut O) -> Option<UserDeliver<R>> where O: Connector<C, A> {
        let (s, q, a) =
            (&mut self.s, &mut self.q, &self.a);

        fsm_turn_x(s, |s| match s {
            CMS::None =>
                if !q.is_empty() { //start connecting, turn over
                    if let Some(a) = a {
                        SxV::S(CMS::Connecting(c.connect(a)))
                    } else {
                        SxV::V(Some(UserDeliver::Err(app_error!(other "Empty peer address"))))
                    }
                } else {
                    SxV::V(None)
                }
            CMS::Connecting(cf) => //poll the cf; if ready, turn over
                match cf.poll() {
                    Ok(Async::NotReady) => SxV::V(None),
                    Ok(Async::Ready(conn)) => SxV::S(CMS::Active(conn)),
                    Err(e) => SxV::SV(CMS::None, Some(UserDeliver::Err(e.into())))
                }
            CMS::Active(conn) =>
                match q.pop_front() {
                    Some(req) => {
                        let w = match conn.start_send(req) {
                            Ok(AsyncSink::Ready) =>
                                match conn.poll_complete() {
                                    Err(e) => Err(e), //publish error
                                    _ => Ok(())
                                }
                            Ok(AsyncSink::NotReady(req)) =>
                                Ok(q.push_front(req)),
                            Err(e) =>
                                Err(e)
                        };
                        SxV::V(
                            if let Err(e) = w {
                                Some(UserDeliver::Err(e))
                            } else {
                                match conn.poll() {
                                    Ok(Async::Ready(Some(rv))) => Some(UserDeliver::Message(rv)),
                                    Ok(Async::Ready(None)) => Some(UserDeliver::EndOfStream),
                                    Ok(Async::NotReady) => None,
                                    Err(e) => Some(UserDeliver::Err(e))
                                }
                            }
                        )
                    }
                    None =>
                        SxV::V(match conn.poll() {
                            Ok(Async::Ready(Some(rv))) => Some(UserDeliver::Message(rv)),
                            Ok(Async::Ready(None)) => Some(UserDeliver::EndOfStream),
                            Ok(Async::NotReady) => None,
                            Err(e) => Some(UserDeliver::Err(e))
                        })
                }
        })
    }

    fn q_size(&self) -> usize {
        self.q.len()
    }
}

type NnCm = CM<nn::Connection_, NnaQ, SocketAddr>;
type DtCm = CM<dt::Connection_, DtaQ, SocketAddr>;
type Channel = usize;

#[derive(Debug)]
pub enum MdxQ {
    NN(Channel, Option<SocketAddr>, NnaQ),
    DT(Channel, Option<SocketAddr>, DtaQ)
}

#[derive(Debug)]
pub enum MdxR {
    NN(Channel, NnaR),
    DT(Channel, DtaR)
}

/// Connection multiplexer/demux
pub struct Mdx {
    nn: Vec<NnCm>,
    dt: Vec<DtCm>,
    u: ProtoFrontEndSourceQ<MdxR>,
    c: NatConnector,
    nna: SocketAddr
}

impl Mdx {
    pub fn new(n_nn: usize,
               n_dt: usize,
               session_data: SessionData,
               nn_address: SocketAddr,
               init_nat: Vec<(SocketAddr, SocketAddr)>) -> Mdx {
        let mut rv = Mdx {
            nn: vec![],
            dt: vec![],
            u: ProtoFrontEndSourceQ::new(),
            nna: nn_address,
            c: NatConnector::new(session_data, init_nat)
        };
        for _ in 0..n_nn {
            rv.nn.push(CM::new())
        }
        for _ in 0..n_dt {
            rv.dt.push(CM::new())
        }
        rv
    }

    #[inline]
    fn handle_nn(&mut self, channel: usize) -> usize {
        if let Some(ud) = self.nn[channel].run(&mut self.c) {
            self.u.push(ud.map(|r| MdxR::NN(channel, r)))
        }

        self.nn[channel].q_size()
    }

    #[inline]
    fn handle_dt(&mut self, channel: usize) -> usize {
        if let Some(ud) = self.dt[channel].run(&mut self.c) {
            self.u.push(ud.map(|r| MdxR::DT(channel, r)))
        }

        self.dt[channel].q_size()
    }

    fn run(&mut self) -> usize {
        let mut rv = 0;
        for ch in 0..self.nn.len() {
            rv += self.handle_nn(ch)
        }
        for ch in 0..self.dt.len() {
            rv += self.handle_dt(ch)
        }
        rv
    }

    #[inline]
    fn nn_a(&self, a: Option<SocketAddr>) -> Option<SocketAddr> {
        a.or_else(|| Some(self.nna.clone()))
    }
}

impl Sink for Mdx {
    type SinkItem = MdxQ;
    type SinkError = Error;

    fn start_send(&mut self, req: MdxQ) -> Result<AsyncSink<MdxQ>> {
        match req {
            MdxQ::NN(channel, a, req) =>
                match { let a = self.nn_a(a); self.nn[channel].push_q((req, a)) } {
                    AsyncSink::Ready => {
                        self.handle_nn(channel);
                        Ok(AsyncSink::Ready)
                    }
                    AsyncSink::NotReady((req, a)) =>
                        Ok(AsyncSink::NotReady(MdxQ::NN(channel, a, req)))
                }
            MdxQ::DT(channel, a, req) =>
                match self.dt[channel].push_q((req, a)) {
                    AsyncSink::Ready => {
                        self.handle_dt(channel);
                        Ok(AsyncSink::Ready)
                    }
                    AsyncSink::NotReady((req, a)) =>
                        Ok(AsyncSink::NotReady(MdxQ::DT(channel, a, req)))
                }
        }
    }

    fn poll_complete(&mut self) -> Result<Async<()>> {
        if self.run() == 0 {
            Ok(Async::Ready(()))
        } else {
            Ok(Async::NotReady)
        }
    }

    fn close(&mut self) -> Result<Async<()>> {
        self.poll_complete()
    }
}

impl Stream for Mdx {
    type Item = MdxR;
    type Error = Error;

    fn poll(&mut self) -> Result<Async<Option<MdxR>>> {
        let _ = self.run();
        self.u.take_deliver()
    }
}

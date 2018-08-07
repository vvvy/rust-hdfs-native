
use std::collections::VecDeque;
use std::fmt::Debug;
use std::marker::PhantomData;

use futures::{Stream, Sink, Async, AsyncSink};
use *;

#[derive(Debug)]
pub enum NetEvent<NR> {
    Idle,
    Incoming(NR),
    Err(Error),
    EndOfStream
}

#[derive(Debug)]
pub enum UserEvent<UQ> {
    ///NOTE: delivered outside of the context of a task, thus should only be used to
    ///set flow control flags
    Init,
    Message(UQ),
    Flush
}

#[derive(Debug)]
pub enum UserDeliver<UR> {
    Message(UR),
    EndOfStream,
    Err(Error)
}

impl<UR> UserDeliver<UR> {
    pub fn map<UR2>(self, f: impl FnOnce(UR) -> UR2) -> UserDeliver<UR2> {
        match self {
            UserDeliver::Message(m) => UserDeliver::Message(f(m)),
            UserDeliver::EndOfStream => UserDeliver::EndOfStream,
            UserDeliver::Err(e) => UserDeliver::Err(e)
        }
    }
}

#[derive(Debug)]
pub struct NetAction<NQ> {
    /// Message to send out to the network
    send: Option<NQ>,
    /// Receiver state
    receive: Option<bool>
}

impl<NQ> NetAction<NQ> {
    /// Neutral (zero) action value, a no-op by itself
    #[inline]
    pub fn z() -> NetAction<NQ> {
        NetAction { send: None, receive: None }
    }
    #[inline]
    pub fn send(self, req: NQ) -> NetAction<NQ> {
        NetAction { send: Some(req), ..self }
    }
    #[inline]
    pub fn recv(self, v: bool) -> NetAction<NQ> {
        NetAction { receive: Some(v), ..self }
    }
}

#[derive(Debug)]
pub struct UserAction<UR> {
    /// Message to deliver to the user
    deliver: Option<UserDeliver<UR>>,
    /// Receiver state (== User can send)
    accept: Option<bool>,
    /// Sending complete
    send_complete: Option<bool>
}

impl<UR> UserAction<UR> {
    #[inline]
    pub fn new(deliver: Option<UserDeliver<UR>>, accept: Option<bool>, send_complete: Option<bool>) -> UserAction<UR> {
        UserAction { deliver, accept, send_complete }
    }
    /// Neutral (zero) action value, a no-op by itself
    #[inline]
    pub fn z() -> UserAction<UR> {
        UserAction { deliver: None, accept: None, send_complete: None }
    }
    #[inline]
    pub fn deliver(self, rsp: UR) -> UserAction<UR> {
        UserAction { deliver: Some(UserDeliver::Message(rsp)), ..self }
    }
    #[inline]
    pub fn eos(self) -> UserAction<UR> {
        UserAction { deliver: Some(UserDeliver::EndOfStream), ..self }
    }
    #[inline]
    pub fn err(self, e: Error) -> UserAction<UR> {
        UserAction { deliver: Some(UserDeliver::Err(e)), ..self }
    }
    #[inline]
    pub fn accept(self, v: bool) -> UserAction<UR> {
        UserAction { accept: Some(v), ..self }

    }
    #[inline]
    pub fn send_complete(self, v: bool) -> UserAction<UR> {
        UserAction { send_complete: Some(v), ..self }
    }
}

#[derive(Debug)]
pub struct Action<NQ, UR> {
    n: NetAction<NQ>,
    u: UserAction<UR>
}

impl<NQ, UR> Action<NQ, UR> {
    /// Neutral (zero) action value, a no-op by itself
    #[inline]
    pub fn z() -> Action<NQ, UR> {
        Action {
            n: NetAction::z(),
            u: UserAction::z()
        }
    }
    #[inline]
    pub fn send(self, req: NQ) -> Action<NQ, UR> {
        Action {
            n: self.n.send(req),
            ..self
        }
    }
    #[inline]
    pub fn recv(self, v: bool) -> Action<NQ, UR> {
        Action {
            n: self.n.recv(v),
            ..self
        }
    }
    #[inline]
    pub fn deliver(self, rsp: UR) -> Action<NQ, UR> {
        Action {
            u: self.u.deliver(rsp),
            ..self
        }
    }
    #[inline]
    pub fn eos(self) -> Action<NQ, UR> {
        Action {
            u: self.u.eos(),
            ..self
        }
    }
    #[inline]
    pub fn err(self, e: Error) -> Action<NQ, UR> {
        Action {
            u: self.u.err(e),
            ..self
        }
    }
    #[inline]
    pub fn accept(self, v: bool) -> Action<NQ, UR> {
        Action {
            u: self.u.accept(v),
            ..self
        }
    }
    #[inline]
    pub fn send_complete(self, v: bool) -> Action<NQ, UR> {
        Action {
            u: self.u.send_complete(v),
            ..self
        }
    }
    #[inline]
    pub fn bits(self, recv: bool, accept: bool, send_complete: bool) -> Action<NQ, UR> {
        self.recv(recv).accept(accept).send_complete(send_complete)
    }
}

/*
macro_rules! proto_action {
    { zero, $($fr:ident : $vr:expr),+ } => { proto_action_inner!(Action::z(), $($fr:ident : $vr:expr),+) };
    { bits $b:tt, $($fr:ident : $vr:expr),+ } => { proto_action_inner!(proto_bits!(Action::z(), $b), $($fr:ident : $vr:expr),+) };
    { $e:expr } => { $e };
}

macro_rules! proto_action_inner {
    { $e:expr, $f:ident : $v:expr, $($fr:ident : $vr:expr),+ } => { proto_action_inner!($e.$f($v), $($fr:ident : $vr:expr),+) };
    { $e:expr } => { $e };
}
*/

#[macro_export]
macro_rules! proto_bits {
    ($a:expr, +recv, +accept, +send_complete) => { $a.bits(true, true, true) };
    ($a:expr, -recv, +accept, +send_complete) => { $a.bits(false, true, true) };
    ($a:expr, +recv, -accept, +send_complete) => { $a.bits(true, false, true) };
    ($a:expr, -recv, -accept, +send_complete) => { $a.bits(false, false, true) };
    ($a:expr, +recv, +accept, -send_complete) => { $a.bits(true, true, false) };
    ($a:expr, -recv, +accept, -send_complete) => { $a.bits(false, true, false) };
    ($a:expr, +recv, -accept, -send_complete) => { $a.bits(true, false, false) };
    ($a:expr, -recv, -accept, -send_complete) => { $a.bits(false, false, false) };
}


struct NetSide<N, NQ, NR> {
    io: N,
    sending: bool,
    receiving: bool,
    nq_type: PhantomData<NQ>,
    nr_type: PhantomData<NR>
}

impl<N, NQ, NR> NetSide<N, NQ, NR> where
    N: Sink<SinkItem=NQ, SinkError=Error> + Stream<Item=NR, Error=Error>,
    NQ: Debug {
    fn new(io: N) -> NetSide<N, NQ, NR> {
        NetSide {
            io,
            sending: false,
            receiving: false,
            nq_type: PhantomData,
            nr_type: PhantomData
        }
    }

    #[inline]
    fn handle_na(&mut self, NetAction { send, receive }: NetAction<NQ>) -> Option<Error> {
        if let Some(receive) = receive { self.receiving = receive }
        match send {
            Some(m) => match self.io.start_send(m) {
                Ok(AsyncSink::Ready) => {
                    self.sending = true;
                    None
                }
                Ok(AsyncSink::NotReady(m)) =>
                //TODO this behaviour could be redesigned, to support buffered blind sending
                //introduce N.A. SendBuffered, Flush and N.E. SendOverflow(m)
                    Some(app_error!(other "Sink overflow on {:?}", m)),
                Err(e) =>
                    Some(e)
            }
            None =>
                None
        }
    }

    #[inline]
    fn poll_recv(&mut self) -> Option<NetEvent<NR>> {
        if self.receiving {
            match self.io.poll() {
                Ok(Async::Ready(Some(r))) => Some(NetEvent::Incoming(r)),
                Ok(Async::Ready(None)) => Some(NetEvent::EndOfStream),
                Ok(Async::NotReady) => None,
                Err(e) => Some(NetEvent::Err(e))
            }
        } else {
            None
        }
    }

    #[inline]
    fn poll_send(&mut self) -> Option<NetEvent<NR>> {
        if self.sending {
            match self.io.poll_complete() {
                Ok(Async::Ready(())) => {
                    self.sending = false;
                    Some(NetEvent::Idle)
                }
                Ok(Async::NotReady) =>
                    None,
                Err(e) =>
                    Some(NetEvent::Err(e))
            }
        } else {
            None
        }
    }
}

pub struct UserSide<UR> {
    send_complete: bool,
    accept: bool,
    deliver: VecDeque<UserDeliver<UR>>
}

impl<UR> UserSide<UR> {
    pub fn new() -> UserSide<UR> {
        UserSide {
            send_complete: false,
            accept: false,
            deliver: VecDeque::new()
        }
    }

    #[inline]
    pub fn handle_ua(&mut self, UserAction { deliver, accept, send_complete }: UserAction<UR>) {
        if let Some(accept) = accept { self.accept = accept }
        if let Some(send_complete) = send_complete { self.send_complete = send_complete }

        if let Some(m) = deliver {
            self.deliver.push_back(m)
        }
    }

    #[inline]
    pub fn push_error(&mut self, e: Error) {
        self.deliver.push_back(UserDeliver::Err(e))
    }

    #[inline]
    pub fn take_error(&mut self) -> Result<()> {
        if let Some(&UserDeliver::Err(..)) = self.deliver.front() {
            match self.deliver.pop_front() {
                Some(UserDeliver::Err(e)) => Err(e),
                Some(other) => { //should never happen
                    self.deliver.push_front(other);
                    Ok(())
                }
                None => //should never happen
                    Ok(())
            }
        } else {
            Ok(())
        }
    }

    #[inline]
    pub fn take_deliver(&mut self) -> Result<Async<Option<UR>>> {
        match self.deliver.pop_front() {
            None =>
                Ok(Async::NotReady),
            Some(UserDeliver::Message(r)) =>
                Ok(Async::Ready(Some(r))),
            Some(UserDeliver::EndOfStream) =>
                Ok(Async::Ready(None)),
            Some(UserDeliver::Err(e)) =>
                Err(e),
        }
    }
}

pub trait ProtoFsm {
    type NQ: Debug;
    type NR: Debug;
    type UQ: Debug;
    type UR: Debug;
    fn handle_n(&mut self, ne: NetEvent<Self::NR>) -> Action<Self::NQ, Self::UR>;
    fn handle_u(&mut self, ue: UserEvent<Self::UQ>) -> Action<Self::NQ, Self::UR>;
}

//--------------------------------------------------------------------------------------------------

pub struct ProtocolLayer<N, P> where
    P: ProtoFsm,
    N: Sink<SinkItem=P::NQ, SinkError=Error> + Stream<Item=P::NR, Error=Error> {
    n: NetSide<N, <P as ProtoFsm>::NQ, <P as ProtoFsm>::NR>,
    u: UserSide<<P as ProtoFsm>::UR>,
    fsm: P
}

impl<N, P> ProtocolLayer<N, P> where
    P: ProtoFsm,
    N: Sink<SinkItem=P::NQ, SinkError=Error> + Stream<Item=P::NR, Error=Error>,
    P::NQ: Debug {
    pub fn new(io: N, fsm: P) -> ProtocolLayer<N, P> {
        let mut rv = ProtocolLayer {
            n: NetSide::new(io),
            u: UserSide::new(),
            fsm
        };
        rv.handle_ue(UserEvent::Init);
        rv
    }

    pub fn into_inner(self) -> N { self.n.io }

    fn handle_a(&mut self, a: Action<P::NQ, P::UR>) {
        let Action { n, u } = a;

        self.u.handle_ua(u);
        if let Some(e) = self.n.handle_na(n) {
            self.u.push_error(e)
        }

        self.n_poll_send();
        self.n_poll_recv();
    }

    #[inline]
    fn handle_ne(&mut self, ne: NetEvent<P::NR>) {
        trace!("handle_ne: event {:?}", ne);
        let a = self.fsm.handle_n(ne);
        trace!("handle_ne: action {:?}", a);
        self.handle_a(a)
    }
    #[inline]
    fn handle_ue(&mut self, ue: UserEvent<P::UQ>) {

        trace!("handle_ue: event {:?}", ue);
        let a = self.fsm.handle_u(ue);
        trace!("handle_ue: action {:?}", a);
        //let a = trace_ar!("handle_ue", ue, self.fsm.handle_u(ue));
        self.handle_a(a)
    }

    #[inline]
    fn n_poll_recv(&mut self) {
        if let Some(ne) = self.n.poll_recv() {
            self.handle_ne(ne)
        }
    }

    #[inline]
    fn n_poll_send(&mut self) {
        if let Some(ne) = self.n.poll_send() {
            self.handle_ne(ne)
        }
    }
}

impl<N, P> Stream for ProtocolLayer<N, P> where
    P: ProtoFsm,
    N: Sink<SinkItem=P::NQ, SinkError=Error> + Stream<Item=P::NR, Error=Error>,
    P::NQ: Debug {
    type Item = P::UR;
    type Error = Error;
    fn poll(&mut self) -> Result<Async<Option<P::UR>>> {
        self.n_poll_recv();
        self.n_poll_send();
        self.u.take_deliver()
    }
}

impl<N, P> Sink for ProtocolLayer<N, P> where
    P: ProtoFsm,
    N: Sink<SinkItem=P::NQ, SinkError=Error> + Stream<Item=P::NR, Error=Error>,
    P::NQ: Debug{
    type SinkItem = P::UQ;
    type SinkError = Error;
    fn start_send(&mut self, req: P::UQ) -> Result<AsyncSink<P::UQ>> {
        let rv = if self.u.accept {
            self.handle_ue(UserEvent::Message(req));
            AsyncSink::Ready
        } else {
            AsyncSink::NotReady(req)
        };
        self.u.take_error().map(|_| rv)
    }

    fn poll_complete(&mut self) -> Result<Async<()>> {
        if !self.u.send_complete {
            self.handle_ue(UserEvent::Flush);
        }
        self.u.take_error().map(|_| if self.u.send_complete {
            Async::Ready(())
        } else {
            Async::NotReady
        })
    }
    fn close(&mut self) -> Result<Async<()>> {
        self.poll_complete()
    }
}

//---------------------------------------------------

#[derive(Debug)]
enum RmrState {
    Sending,
    Flushing,
    Receiving,
    EndOfStream
}

/// 'Request => Multiple Responses' behavior. Sends `()` to the underlying stream, then expects
/// a number of `R` messages in response. Responses are read up to and including the last one which
/// `into().1` yields to true for.

pub struct ReqMultiResp<S, R, RR> {
    state: RmrState,
    s: S,
    into: fn(R) -> (RR, bool),
    r_type: PhantomData<R>,
    rr_type: PhantomData<RR>
}

impl<S, R, RR> ReqMultiResp<S, R, RR> {
    pub fn new(s: S, into: fn(R) -> (RR, bool)) -> ReqMultiResp<S, R, RR> {
        ReqMultiResp { s, into, state: RmrState::Sending, r_type: PhantomData, rr_type: PhantomData }
    }
    pub fn into_inner(self) -> S { self.s }
}

impl<S, R, RR> Stream for ReqMultiResp<S, R, RR> where
    S: Stream<Item=R, Error=Error> + Sink<SinkItem=(), SinkError=Error>,
    RR: Debug
{
    type Item = RR;
    type Error = Error;

    fn poll(&mut self) -> Result<Async<Option<RR>>> {
        loop {
            let op = match &self.state {
                RmrState::Sending => match self.s.start_send(()) {
                    Ok(AsyncSink::Ready) => SxV::S(RmrState::Flushing),
                    Ok(AsyncSink::NotReady(_)) => SxV::V(Ok(Async::NotReady)),
                    Err(e) => SxV::V(Err(e))
                }
                RmrState::Flushing => match self.s.poll_complete() {
                    Ok(Async::Ready(())) => SxV::S(RmrState::Receiving),
                    Ok(Async::NotReady) => SxV::V(Ok(Async::NotReady)),
                    Err(e) => SxV::V(Err(e))
                }
                RmrState::Receiving => match self.s.poll() {
                    Ok(Async::Ready(Some(v))) => {
                        let (w, is_last): (RR, bool) = (self.into)(v);
                        if is_last {
                            //futures::task::current().notify();
                            SxV::SV(RmrState::EndOfStream, Ok(Async::Ready(Some(w))))
                        } else {
                            SxV::V(Ok(Async::Ready(Some(w))))
                        }
                    }
                    Ok(Async::Ready(None)) => SxV::V(Err(app_error!(premature eof))),
                    Ok(Async::NotReady) => SxV::V(Ok(Async::NotReady)),
                    Err(e) => SxV::V(Err(e))
                }
                RmrState::EndOfStream =>
                    SxV::V(Ok(Async::Ready(None)))
            };
            trace!("ReqMultiResp: {:?} => {:?}", self.state, op);
            match op {
                SxV::S(s) => self.state = s,
                SxV::V(v) => break v,
                SxV::SV(s, v) => {
                    self.state = s;
                    break v
                }
            }
        }
    }
}




/*
mod p2 {
    use *;
    use futures::prelude::*;

    enum Branch<A, B> {
        Proceed(A),
        Return(B),
        Error(Error)
    }

    pub trait ProtoFsm {
        type NQ;
        type NR;
        type UQ;
        type UR;

        fn uq(uq: Self::UQ) -> Branch<Self::NQ, AsyncSink<Self::UQ>>;
        fn uq_r(uqr: Result<AsyncSink<Self::NQ>>) -> Result<AsyncSink<Self::UQ>>;
        fn flush() -> Branch<(), Async<()>>;
        fn flush_r(Result<Async<()>>) -> Result<Async<()>>;
        fn poll() -> Branch<(), Async<Option<Self::UR>>>;
        fn poll_r(r: Async<Option<Self::NR>>) -> Async<Option<Self::UR>>;
    }

    pub struct ProtocolLayer<N, P> where
        P: ProtoFsm,
        N: Sink<SinkItem=P::NQ, SinkError=IoError> + Stream<Item=P::NR, Error=IoError> {
        n: N,
        fsm: P
    }
}
*/
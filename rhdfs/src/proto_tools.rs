use std::collections::VecDeque;
use std::fmt::Debug;
use std::marker::PhantomData;

use futures::prelude::*; //{Stream, Sink, Async, AsyncSink};
use *;

#[derive(Debug)]
pub enum NetEvent<NR> {
    Init,
    Idle,
    Incoming(NR),
    Err(Error),
    EndOfStream
}

#[derive(Debug)]
pub enum UserEvent<UQ> {
    Message(UQ),
    Flush
}

#[derive(Debug)]
pub enum UserDeliver<UR> {
    Message(UR),
    MessageLast(UR),
    EndOfStream,
    Err(Error)
}

impl<UR> UserDeliver<UR> {
    pub fn map<UR2>(self, f: impl FnOnce(UR) -> UR2) -> UserDeliver<UR2> {
        match self {
            UserDeliver::Message(m) => UserDeliver::Message(f(m)),
            UserDeliver::MessageLast(m) => UserDeliver::MessageLast(f(m)),
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
    pub fn z() -> NetAction<NQ> {  NetAction { send: None, receive: None } }
    #[inline]
    pub fn send(self, req: NQ) -> NetAction<NQ> { NetAction { send: Some(req), ..self } }
    #[inline]
    pub fn recv(self, v: bool) -> NetAction<NQ> { NetAction { receive: Some(v), ..self } }
}


#[derive(Debug)]
pub struct SinkAction<NQ> {
    n: NetAction<NQ>,
    /// Receiver state (== User can send)
    accept: Option<bool>,
    /// Sending complete
    send_complete: Option<bool>
}

impl<NQ> SinkAction<NQ> {
    #[inline]
    pub fn z() -> SinkAction<NQ> { SinkAction { n: NetAction::z(), accept: None, send_complete: None } }
    #[inline]
    pub fn send(self, req: NQ) -> SinkAction<NQ> { SinkAction { n: self.n.send(req), ..self } }
    #[inline]
    pub fn recv(self, v: bool) -> SinkAction<NQ> { SinkAction { n: self.n.recv(v), ..self } }
    #[inline]
    pub fn accept(self, v: bool) -> SinkAction<NQ> { SinkAction { accept: Some(v), ..self } }
    #[inline]
    pub fn send_complete(self, v: bool) -> SinkAction<NQ> { SinkAction { send_complete: Some(v), ..self } }
    #[inline]
    pub fn bits(self, (recv, accept, send_complete): (bool, bool, bool)) -> SinkAction<NQ> {
        self.recv(recv).accept(accept).send_complete(send_complete)
    }
}

#[derive(Debug)]
pub struct SourceAction<NQ, UR> {
    n: NetAction<NQ>,
    /// Delivery to the user
    deliver: Option<UserDeliver<UR>>
}

impl<NQ, UR> SourceAction<NQ, UR> {
    #[inline]
    pub fn z() -> SourceAction<NQ, UR> { SourceAction { n: NetAction::z(), deliver:None } }
    #[inline]
    pub fn send(self, req: NQ) -> SourceAction<NQ, UR> { SourceAction { n: self.n.send(req), ..self } }
    #[inline]
    pub fn recv(self, v: bool) -> SourceAction<NQ, UR> { SourceAction { n: self.n.recv(v), ..self } }
    #[inline]
    pub fn bits(self, recv: bool) -> SourceAction<NQ, UR> { self.recv(recv) }
    #[inline]
    pub fn deliver(self, rsp: UR) -> SourceAction<NQ, UR> { SourceAction { deliver: Some(UserDeliver::Message(rsp)), ..self } }
    #[inline]
    pub fn deliver_last(self, rsp: UR) -> SourceAction<NQ, UR> { SourceAction { deliver: Some(UserDeliver::MessageLast(rsp)), ..self } }
    #[inline]
    pub fn eos(self) -> SourceAction<NQ, UR> { SourceAction { deliver: Some(UserDeliver::EndOfStream), ..self } }
    #[inline]
    pub fn err(self, e: Error) -> SourceAction<NQ, UR> { SourceAction { deliver: Some(UserDeliver::Err(e)), ..self } }
}

#[derive(Debug)]
pub struct Action<NQ, UR> {
    n: NetAction<NQ>,
    /// Delivery to the user
    deliver: Option<UserDeliver<UR>>,
    /// Receiver state (== User can send)
    accept: Option<bool>,
    /// Sending complete
    send_complete: Option<bool>
}

impl<NQ, UR> Action<NQ, UR> {
    #[inline]
    pub fn z() -> Action<NQ, UR> { Action { n: NetAction::z(), deliver:None, accept: None, send_complete: None } }
    #[inline]
    pub fn send(self, req: NQ) -> Action<NQ, UR> { Action { n: self.n.send(req), ..self } }
    #[inline]
    pub fn recv(self, v: bool) -> Action<NQ, UR> { Action { n: self.n.recv(v), ..self } }
    #[inline]
    pub fn accept(self, v: bool) -> Action<NQ, UR> { Action { accept: Some(v), ..self } }
    #[inline]
    pub fn send_complete(self, v: bool) -> Action<NQ, UR> { Action { send_complete: Some(v), ..self } }
    #[inline]
    pub fn bits(self, (recv, accept, send_complete): (bool, bool, bool)) -> Action<NQ, UR> {
        self.recv(recv).accept(accept).send_complete(send_complete)
    }
    #[inline]
    pub fn deliver(self, rsp: UR) -> Action<NQ, UR> { Action { deliver: Some(UserDeliver::Message(rsp)), ..self } }
    #[inline]
    pub fn eos(self) -> Action<NQ, UR> { Action { deliver: Some(UserDeliver::EndOfStream), ..self } }
    #[inline]
    pub fn err(self, e: Error) -> Action<NQ, UR> { Action { deliver: Some(UserDeliver::Err(e)), ..self } }
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
    (+recv, +accept, +send_complete) => { (true, true, true) };
    (-recv, +accept, +send_complete) => { (false, true, true) };
    (+recv, -accept, +send_complete) => { (true, false, true) };
    (-recv, -accept, +send_complete) => { (false, false, true) };
    (+recv, +accept, -send_complete) => { (true, true, false) };
    (-recv, +accept, -send_complete) => { (false, true, false) };
    (+recv, -accept, -send_complete) => { (true, false, false) };
    (-recv, -accept, -send_complete) => { (false, false, false) };
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
    fn reset(&mut self) {
        self.sending = false;
        self.receiving = false;
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
                Ok(AsyncSink::NotReady(m)) => {
                    //TODO this behaviour could be redesigned, to support buffered blind sending
                    //introduce N.A. SendBuffered, Flush and N.E. SendOverflow(m)
                    self.reset();
                    Some(app_error!(other "Sink overflow on {:?}", m))
                }
                Err(e) => {
                    self.reset();
                    Some(e)
                }
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


//--------------------------------------------------------------------------------------------------

pub trait ProtoFsmSource {
    type NQ: Debug;
    type NR: Debug;
    type UR: Debug;
    fn handle_n(&mut self, ne: NetEvent<Self::NR>) -> SourceAction<Self::NQ, Self::UR>;
}

pub trait ProtoFsmSink {
    type NQ: Debug;
    type NR: Debug;
    type UQ: Debug;
    fn handle_n(&mut self, ne: NetEvent<Self::NR>) -> SinkAction<Self::NQ>;
    fn handle_u(&mut self, ue: UserEvent<Self::UQ>) -> SinkAction<Self::NQ>;
}

pub trait ProtoFsm {
    type NQ: Debug;
    type NR: Debug;
    type UQ: Debug;
    type UR: Debug;
    fn handle_n(&mut self, ne: NetEvent<Self::NR>) -> Action<Self::NQ, Self::UR>;
    fn handle_u(&mut self, ue: UserEvent<Self::UQ>) -> Action<Self::NQ, Self::UR>;
}

//==================================================================================================


pub trait ProtoFrontEndBase {
    type NQ: Debug;
    type NR: Debug;
    fn handle_n(&mut self, ne: NetEvent<Self::NR>) -> NetAction<Self::NQ>;
    fn take_error(&mut self) -> Result<()>;
    fn push_error(&mut self, e: Error);
}

pub trait ProtoFrontEndSource: ProtoFrontEndBase {
    type UR: Debug;
    fn take_deliver(&mut self) -> Result<Async<Option<Self::UR>>>;
}

pub trait ProtoFrontEndFuture: ProtoFrontEndBase {
    type UR: Debug;
    fn take_deliver(&mut self) -> Result<Async<Self::UR>>;
}

pub trait ProtoFrontEndSink: ProtoFrontEndBase {
    type UQ: Debug;
    fn is_accept(&self) -> bool;
    fn is_send_complete(&self) -> bool;
    fn handle_u(&mut self, ue: UserEvent<Self::UQ>) -> NetAction<Self::NQ>;
}

pub struct ProtoLayer<N, P> where
    P: ProtoFrontEndBase,
    N: Sink<SinkItem=P::NQ, SinkError=Error> + Stream<Item=P::NR, Error=Error> {
    n: NetSide<N, P::NQ, P::NR>,
    u: P,
    init: bool
}

impl<N, P> ProtoLayer<N, P> where
    P: ProtoFrontEndBase,
    N: Sink<SinkItem=P::NQ, SinkError=Error> + Stream<Item=P::NR, Error=Error> {

    pub fn new(io: N, u: P) -> ProtoLayer<N, P> {
        ProtoLayer {
            n: NetSide::new(io),
            u,
            init: false
        }
    }

    pub fn into_inner(self) -> N { self.n.io }

    #[inline]
    fn handle_na_int(&mut self, na: NetAction<P::NQ>) {
        if let Some(e) = self.n.handle_na(na) {
            self.u.push_error(e)
        }
    }
    #[inline]
    fn handle_ne_int(&mut self, ne: NetEvent<P::NR>) {
        let na = self.u.handle_n(ne);
        self.handle_na_int(na)
    }

    fn n_poll(&mut self) {
        while
            self.n.poll_send().map_or_else(
                || false,
                |ne| { self.handle_ne_int(ne); true}
            )
            ||
            self.n.poll_recv().map_or_else(
                || false,
                |ne| { self.handle_ne_int(ne); true}
            )
            { }
    }

    #[inline]
    fn handle_ne(&mut self, ne: NetEvent<P::NR>) {
        self.handle_ne_int(ne);
        self.n_poll();
    }

    #[inline]
    fn handle_na(&mut self, na: NetAction<P::NQ>) {
        self.handle_na_int(na);
        self.n_poll();
    }

    #[inline]
    fn n_init(&mut self) {
        if !self.init {
            self.init = true;
            self.handle_ne(NetEvent::Init)
        }
    }

}

impl<N, P> Stream for ProtoLayer<N, P> where
    P: ProtoFrontEndSource,
    N: Sink<SinkItem=P::NQ, SinkError=Error> + Stream<Item=P::NR, Error=Error> {
    type Item = P::UR;
    type Error = Error;
    fn poll(&mut self) -> Result<Async<Option<P::UR>>> {
        self.n_init();
        self.n_poll();
        self.u.take_deliver()
    }
}

impl<N, P> Future for ProtoLayer<N, P> where
    P: ProtoFrontEndFuture,
    N: Sink<SinkItem=P::NQ, SinkError=Error> + Stream<Item=P::NR, Error=Error> {
    type Item = P::UR;
    type Error = Error;
    fn poll(&mut self) -> Result<Async<P::UR>> {
        self.n_init();
        self.n_poll();
        self.u.take_deliver()
    }
}

impl<N, P> ProtoLayer<N, P> where
    P: ProtoFrontEndSink,
    N: Sink<SinkItem=P::NQ, SinkError=Error> + Stream<Item=P::NR, Error=Error> {

    #[inline]
    fn handle_ue(&mut self, ue: UserEvent<P::UQ>) {
        let a = self.u.handle_u(ue);
        self.handle_na(a)
    }
}

impl<N, P> Sink for ProtoLayer<N, P> where
    P: ProtoFrontEndSink,
    N: Sink<SinkItem=P::NQ, SinkError=Error> + Stream<Item=P::NR, Error=Error> {
    type SinkItem = P::UQ;
    type SinkError = Error;
    fn start_send(&mut self, req: P::UQ) -> Result<AsyncSink<P::UQ>> {
        self.n_init();
        let rv = if self.u.is_accept() {
            self.handle_ue(UserEvent::Message(req));
            AsyncSink::Ready
        } else {
            AsyncSink::NotReady(req)
        };
        self.u.take_error().map(|_| rv)
    }

    fn poll_complete(&mut self) -> Result<Async<()>> {
        self.n_init();
        if !self.u.is_send_complete() {
            self.handle_ue(UserEvent::Flush);
        }
        self.u.take_error().map(|_| if self.u.is_send_complete() {
            Async::Ready(())
        } else {
            Async::NotReady
        })
    }
    fn close(&mut self) -> Result<Async<()>> {
        self.poll_complete()
    }
}


/// Source-side queue implementation
pub struct ProtoFrontEndSourceQ<UR> {
    deliver: VecDeque<UserDeliver<UR>>,
    finished: bool
}

impl<UR> ProtoFrontEndSourceQ<UR> {
    pub fn new() -> ProtoFrontEndSourceQ<UR> {
        ProtoFrontEndSourceQ { deliver: VecDeque::new(), finished: false }
    }

    #[inline]
    fn take_error(&mut self) -> Result<()> {
        if self.finished {
            return Ok(())
        } else if let Some(&UserDeliver::Err(..)) = self.deliver.front() {
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
    fn push_error(&mut self, e: Error) {
        self.deliver.push_back(UserDeliver::Err(e))
    }

    #[inline]
    pub fn push(&mut self, ud: UserDeliver<UR>) {
        self.deliver.push_back(ud)
    }

    #[inline]
    pub fn take_deliver(&mut self) -> Result<Async<Option<UR>>> {
        if self.finished {
            Ok(Async::Ready(None))
        } else {
            match self.deliver.pop_front() {
                None =>
                    Ok(Async::NotReady),
                Some(UserDeliver::Message(r)) =>
                    Ok(Async::Ready(Some(r))),
                Some(UserDeliver::MessageLast(r)) => {
                    self.finished = true;
                    Ok(Async::Ready(Some(r)))
                }
                Some(UserDeliver::EndOfStream) => {
                    self.finished = true;
                    Ok(Async::Ready(None))
                }
                Some(UserDeliver::Err(e)) =>
                    Err(e),
            }
        }

    }
}


pub struct ProtoFrontEndSourceImpl<P> where P: ProtoFsmSource {
    q: ProtoFrontEndSourceQ<P::UR>,
    p: P
}

impl<P> ProtoFrontEndSourceImpl<P> where P: ProtoFsmSource {
    fn new(p: P) -> ProtoFrontEndSourceImpl<P> { ProtoFrontEndSourceImpl { q: ProtoFrontEndSourceQ::new(), p } }
}

impl<P> ProtoFrontEndBase for ProtoFrontEndSourceImpl<P> where
    P: ProtoFsmSource {
    type NQ = P::NQ;
    type NR = P::NR;

    #[inline]
    fn handle_n(&mut self, ne: NetEvent<P::NR>) -> NetAction<P::NQ> {
        let SourceAction { n, deliver } = self.p.handle_n(ne);
        if let Some(deliver) = deliver {
            self.q.push(deliver)
        }
        n
    }

    #[inline]
    fn take_error(&mut self) -> Result<()> {
        self.q.take_error()
    }

    #[inline]
    fn push_error(&mut self, e: Error) {
        self.q.push_error(e)
    }
}

impl<P> ProtoFrontEndSource for ProtoFrontEndSourceImpl<P> where
    P: ProtoFsmSource {
    type UR = P::UR;

    fn take_deliver(&mut self) -> Result<Async<Option<P::UR>>> {
        self.q.take_deliver()
    }
}

pub struct ProtoFrontEndImpl<P> where P: ProtoFsm {
    q: ProtoFrontEndSourceQ<P::UR>,
    accept: bool,
    send_complete: bool,
    p: P
}

impl<P> ProtoFrontEndImpl<P> where P: ProtoFsm {
    fn new(p: P) -> ProtoFrontEndImpl<P> {
        ProtoFrontEndImpl { q: ProtoFrontEndSourceQ::new(), accept: false, send_complete: false, p }
    }

    fn handle_a(&mut self, a: Action<P::NQ, P::UR>) -> NetAction<P::NQ> {
        let Action { n, deliver, accept, send_complete } = a;
        if let Some(deliver) = deliver {
            self.q.push(deliver)
        }
        if let Some(send_complete) = send_complete {
            self.send_complete = send_complete
        }
        if let Some(accept) = accept {
            self.accept = accept
        }
        n
    }
}

impl<P> ProtoFrontEndBase for ProtoFrontEndImpl<P> where
    P: ProtoFsm {
    type NQ = P::NQ;
    type NR = P::NR;

    #[inline]
    fn handle_n(&mut self, ne: NetEvent<P::NR>) -> NetAction<P::NQ> {
        let a = self.p.handle_n(ne);
        self.handle_a(a)
    }

    #[inline]
    fn take_error(&mut self) -> Result<()> {
        self.q.take_error()
    }

    #[inline]
    fn push_error(&mut self, e: Error) {
        self.q.push_error(e)
    }
}

impl<P> ProtoFrontEndSource for ProtoFrontEndImpl<P> where
    P: ProtoFsm {
    type UR = P::UR;

    #[inline]
    fn take_deliver(&mut self) -> Result<Async<Option<P::UR>>> {
        self.q.take_deliver()
    }
}


impl<P> ProtoFrontEndSink for ProtoFrontEndImpl<P> where
    P: ProtoFsm {
    type UQ = P::UQ;

    fn is_accept(&self) -> bool {
        self.accept
    }

    fn is_send_complete(&self) -> bool {
        self.send_complete
    }

    fn handle_u(&mut self, ue: UserEvent<P::UQ>) -> NetAction<P::NQ> {
        let a = self.p.handle_u(ue);
        self.handle_a(a)
    }
}

pub struct ProtoFrontEndSinkImpl<P> where P: ProtoFsmSink {
    e: Option<Error>,
    accept: bool,
    send_complete: bool,
    p: P
}

impl<P> ProtoFrontEndSinkImpl<P> where P: ProtoFsmSink {
    fn new(p: P) -> ProtoFrontEndSinkImpl<P> {
        ProtoFrontEndSinkImpl { e: None, accept: false, send_complete: false, p }
    }

    fn handle_a(&mut self, a:SinkAction<P::NQ>) -> NetAction<P::NQ> {
        let SinkAction { n, accept, send_complete } = a;
        if let Some(send_complete) = send_complete {
            self.send_complete = send_complete
        }
        if let Some(accept) = accept {
            self.accept = accept
        }
        n
    }
}

impl<P> ProtoFrontEndBase for ProtoFrontEndSinkImpl<P> where
    P: ProtoFsmSink {
    type NQ = P::NQ;
    type NR = P::NR;

    #[inline]
    fn handle_n(&mut self, ne: NetEvent<P::NR>) -> NetAction<P::NQ> {
        let a = self.p.handle_n(ne);
        self.handle_a(a)
    }

    #[inline]
    fn take_error(&mut self) -> Result<()> {
        self.e.take().map_or_else(|| Ok(()), |e| Err(e))
    }

    #[inline]
    fn push_error(&mut self, e: Error) {
        self.e = Some(e)
    }
}


impl<P> ProtoFrontEndSink for ProtoFrontEndSinkImpl<P> where
    P: ProtoFsmSink {
    type UQ = P::UQ;

    fn is_accept(&self) -> bool {
        self.accept
    }

    fn is_send_complete(&self) -> bool {
        self.send_complete
    }

    fn handle_u(&mut self, ue: UserEvent<P::UQ>) -> NetAction<P::NQ> {
        let a = self.p.handle_u(ue);
        self.handle_a(a)
    }
}


struct ProtoFrontEndFutureImpl<P> where P: ProtoFsmSource {
    d: Option<UserDeliver<P::UR>>,
    p: P
}

impl<P> ProtoFrontEndBase for ProtoFrontEndFutureImpl<P> where
    P: ProtoFsmSource {
    type NQ = P::NQ;
    type NR = P::NR;

    #[inline]
    fn handle_n(&mut self, ne: NetEvent<P::NR>) -> NetAction<P::NQ> {
        let SourceAction { n, deliver } = self.p.handle_n(ne);
        if let Some(deliver) = deliver {
            self.d = Some(deliver)
        }
        n
    }

    #[inline]
    fn take_error(&mut self) -> Result<()> {
        if let Some(UserDeliver::Err(..)) = &self.d {
            match self.d.take() {
                Some(UserDeliver::Err(e)) => Err(e),
                x =>  { self.d = x; Ok(()) }
            }
        } else {
            Ok(())
        }
    }

    #[inline]
    fn push_error(&mut self, e: Error) {
        self.d = Some(UserDeliver::Err(e))
    }
}

impl<P> ProtoFrontEndFuture for ProtoFrontEndFutureImpl<P> where
    P: ProtoFsmSource {
    type UR = P::UR;

    fn take_deliver(&mut self) -> Result<Async<P::UR>> {
        match self.d.take() {
            Some(UserDeliver::Err(e)) => Err(e),
            Some(UserDeliver::MessageLast(ur)) | Some(UserDeliver::Message(ur)) => Ok(Async::Ready(ur)),
            None => Ok(Async::NotReady),
            Some(UserDeliver::EndOfStream) => Err(app_error!(other "Invalid EndOfStream in ProtoFrontEndFutureImpl"))
        }
    }
}
//====================================================================

pub struct Call<NQ, NR> {
    q: Option<NQ>,
    nr_type: PhantomData<NR>
}

impl<NQ, NR> Call<NQ, NR> {
    fn new(nq: NQ) ->  Call<NQ, NR> {
        Call { q: Some(nq), nr_type: PhantomData }
    }
}

impl<NQ, NR> ProtoFsmSource for Call<NQ, NR> where
    NQ: Debug, NR: Debug {
    type NQ = NQ;
    type NR = NR;
    type UR = NR;

    fn handle_n(&mut self, ne: NetEvent<NR>) -> SourceAction<NQ, NR> {
        fn z<NQ, NR>() -> SourceAction<NQ, NR> { SourceAction::z() }
        match ne {
            NetEvent::Init => self.q.take().map_or_else(|| z(), |nq| z().send(nq)),
            NetEvent::Idle => z().recv(true),
            NetEvent::Incoming(nr) => z().deliver_last(nr).recv(false),
            NetEvent::Err(e) => z().err(e),
            NetEvent::EndOfStream => z().err(app_error!(premature eof))
        }
    }
}

pub trait ChatF {
    type NQ: Debug;
    type NR: Debug;
    type UR: Debug;
    fn start(&mut self) -> Self::NQ;
    fn next(&mut self, nr: Self::NR) -> Result<(Self::UR, Option<Self::NQ>)>;
}

pub struct Chat<F> {
    f: F
}

impl<F> Chat<F> {
    fn new(f: F) ->  Chat<F> {
        Chat { f }
    }
}

impl<F> ProtoFsmSource for Chat<F> where
    F: ChatF {
    type NQ = F::NQ;
    type NR = F::NR;
    type UR = F::UR;

    fn handle_n(&mut self, ne: NetEvent<Self::NR>) -> SourceAction<Self::NQ, Self::UR> {
        fn z<NQ, NR>() -> SourceAction<NQ, NR> { SourceAction::z() }

        match ne {
            NetEvent::Init => z().send(self.f.start()),
            NetEvent::Idle => z().recv(true),
            NetEvent::Incoming(nr) => match self.f.next(nr) {
                Ok((ur, Some(nq))) => z().recv(false).send(nq).deliver(ur),
                Ok((ur, None)) => z().recv(false).deliver_last(ur),
                Err(e) => z().err(e)
            }
            NetEvent::Err(e) => z().err(e),
            NetEvent::EndOfStream => z().err(app_error!(premature eof))
        }
    }
}

pub mod call {
    use super::*;
    pub type T<N, NQ, NR> = ProtoLayer<N, ProtoFrontEndSourceImpl<Call<NQ, NR>>>;
    pub fn new<N, NQ, NR>(io: N, nq: NQ) -> T<N, NQ, NR> where
        N: Sink<SinkItem=NQ, SinkError=Error> + Stream<Item=NR, Error=Error>,
        NQ: Debug, NR: Debug {
        ProtoLayer::new(io, ProtoFrontEndSourceImpl::new(Call::new(nq)))
    }
}

pub mod chat {
    use super::*;
    pub type T<N, F> = ProtoLayer<N, ProtoFrontEndSourceImpl<Chat<F>>>;
    pub fn new<N, F>(io: N, f: F) -> T<N, F> where
        N: Sink<SinkItem=F::NQ, SinkError=Error> + Stream<Item=F::NR, Error=Error>,
        F: ChatF {
        ProtoLayer::new(io, ProtoFrontEndSourceImpl::new(Chat::new(f)))
    }
}

pub mod layer {
    use super::*;
    pub type T<N, P> = ProtoLayer<N, ProtoFrontEndImpl<P>>;
    pub fn new<N, P>(io: N, p: P) -> T<N, P> where
        N: Sink<SinkItem=P::NQ, SinkError=Error> + Stream<Item=P::NR, Error=Error>,
        P: ProtoFsm {
        ProtoLayer::new(io, ProtoFrontEndImpl::new(p))
    }
}

pub mod source_layer {
    use super::*;
    pub type T<N, P> = ProtoLayer<N, ProtoFrontEndSourceImpl<P>>;
    pub fn new<N, P>(io: N, p: P) -> T<N, P> where
        N: Sink<SinkItem=P::NQ, SinkError=Error> + Stream<Item=P::NR, Error=Error>,
        P: ProtoFsmSource {
        ProtoLayer::new(io, ProtoFrontEndSourceImpl::new(p))
    }
}

pub mod sink_layer {
    use super::*;
    pub type T<N, P> = ProtoLayer<N, ProtoFrontEndSinkImpl<P>>;
    pub fn new<N, P>(io: N, p: P) -> T<N, P> where
        N: Sink<SinkItem=P::NQ, SinkError=Error> + Stream<Item=P::NR, Error=Error>,
        P: ProtoFsmSink {
        ProtoLayer::new(io, ProtoFrontEndSinkImpl::new(p))
    }
}

pub mod async_io {
    use super::*;
    use tokio_io::{AsyncRead, AsyncWrite};
    use std::io::{Read, Write, ErrorKind};
    use bytes::Bytes;

    /// Converts a Stream into AsyncRead
    pub struct AsyncReadStream<S> {
        s: S,
        b: Bytes
    }

    impl<S> AsyncReadStream<S> {
        pub fn new(s: S) -> AsyncReadStream<S> { AsyncReadStream { s, b: Bytes::new() } }
        pub fn decons(self) -> (S, Bytes) { (self.s, self.b) }
        fn fill_buffer(&mut self, buf: &mut [u8]) -> usize {
            if self.b.is_empty() {
                0
            } else {
                let l = self.b.len().min(buf.len());
                let b = self.b.split_to(l);
                buf[0..l].copy_from_slice(&b[..]);
                l
            }
        }
    }

    impl<S> Read for AsyncReadStream<S> where S: Stream<Item=Bytes, Error=Error> {
        fn read(&mut self, buf: &mut [u8]) -> IoResult<usize> {
            let l = self.fill_buffer(buf);
            if l > 0 {
                Ok(l)
            } else {
                match self.s.poll() {
                    Ok(Async::Ready(Some(b))) => {
                        self.b = b;
                        Ok(self.fill_buffer(buf))   //if b is empty, it's over
                    }
                    Ok(Async::Ready(None)) =>
                        Ok(self.fill_buffer(buf)),
                    Ok(Async::NotReady) =>
                        Err(ErrorKind::WouldBlock.into()),
                    Err(e) =>
                        Err(e.into())
                }
            }
        }
    }

    impl<S> AsyncRead for AsyncReadStream<S> where S: Stream<Item=Bytes, Error=Error> { }

    enum Flushing {
        Now,
        Later,
        Never
    }

    /// Converts a Sink into AsyncWrite
    pub struct AsyncWriteSink<S> {
        auto_flush: bool,
        s: S,
        pending: Option<Bytes>,
        flushing: bool
    }

    impl<S> AsyncWriteSink<S> where S: Sink<SinkItem=Bytes, SinkError=Error> {
        pub fn new(s: S, auto_flush: bool) -> AsyncWriteSink<S> {
            AsyncWriteSink {
                auto_flush,
                s,
                pending: None,
                flushing: false
            }
        }

        /// returns idle indicator wrapped into Result
        fn turn(&mut self) -> Result<()> {
            if self.flushing {
                match self.s.poll_complete()? {
                    Async::Ready(()) => {
                        self.flushing = false;
                        self.turn()
                    }
                    Async::NotReady =>
                        Ok(())
                }
            } else if let Some(bytes) = self.pending.take() {
                match self.s.start_send(bytes)? {
                    AsyncSink::Ready => if self.auto_flush {
                        self.flushing = true;
                        self.turn()
                    } else {
                        Ok(())
                    }
                    AsyncSink::NotReady(bytes) => {
                        self.pending = Some(bytes);
                        self.flushing = true;
                        self.turn()
                    }
                }
            } else {
                Ok(())
            }
        }
    }

    impl<S> Write for AsyncWriteSink<S> where S: Sink<SinkItem=Bytes, SinkError=Error> {
        fn write(&mut self, buf: &[u8]) -> IoResult<usize> {
            self.turn()?;
            if self.pending.is_none() {
                self.pending = Some(Bytes::from(buf));
                self.turn()?;
                Ok(buf.len())
            } else {
                Err(ErrorKind::WouldBlock.into())
            }

        }

        fn flush(&mut self) -> IoResult<()> {
            self.turn()?;
            if self.pending.is_some() {
                Err(ErrorKind::WouldBlock.into())
            } else {
                if self.flushing {
                    self.turn()?;
                }
                if self.flushing {
                    Err(ErrorKind::WouldBlock.into())
                } else {
                    Ok(())
                }
            }
        }
    }

    impl<S> AsyncWrite for AsyncWriteSink<S> where S: Sink<SinkItem=Bytes, SinkError=Error> {
        fn shutdown(&mut self) -> IoResult<Async<()>> {
            Ok(self.s.close()?)
        }
    }


    #[test]
    fn test_async_write_sink() {
        #[derive(Debug)]
        enum TD {
            StartSend(&'static [u8], bool),
            PollComplete(bool),
            Close(bool)
        }

        struct TestSink {
            c: Vec<TD>
        }

        impl TestSink {
            fn new(mut c: Vec<TD>) -> TestSink {
                c.reverse();
                TestSink { c }
            }

            fn finalize(&self) {
                if !self.c.is_empty() {
                    panic!("Entries not consumed: {:?}", self.c)
                }
            }
        }

        impl Sink for TestSink {
            type SinkItem = Bytes;
            type SinkError = Error;

            fn start_send(&mut self, item: Bytes) -> Result<AsyncSink<Bytes>> {
                let x = self.c.pop();
                if let Some(TD::StartSend(bs, r)) = x {
                    assert_eq!(bs, item);
                    if r { Ok(AsyncSink::Ready) } else { Ok(AsyncSink::NotReady(item)) }
                } else {
                    panic!("got {:?} while expecting StartSend", x)
                }
            }

            fn poll_complete(&mut self) -> Result<Async<()>> {
                let x = self.c.pop();
                if let Some(TD::PollComplete(r)) = x {
                    if r { Ok(Async::Ready(())) } else { Ok(Async::NotReady) }
                } else {
                    panic!("got {:?} while expecting PollComplete", x)
                }
            }

            fn close(&mut self) -> Result<Async<()>> {
                let x = self.c.pop();
                if let Some(TD::Close(r)) = x {
                    if r { Ok(Async::Ready(())) } else { Ok(Async::NotReady) }
                } else {
                    panic!("got {:?} while expecting Close", x)
                }
            }
        }

        //----------------------------------------------------------------------------------
        // no framing
        //----------------------------------------------------------------------------------
        let mut w = AsyncWriteSink::new(TestSink::new(vec![
            TD::StartSend(&[0, 1, 2], true),
            //poll_complete() is redundant here - close() is enough, see close() docs
            TD::Close(true)
        ]), false);
        assert_eq!(w.write(&[0, 1, 2]).unwrap(), 3);
        assert_eq!(w.shutdown().unwrap(), Async::Ready(()));
        w.s.finalize();

        let mut w = AsyncWriteSink::new(TestSink::new(vec![
            TD::StartSend(&[0, 1, 2], true),
            TD::StartSend(&[3, 4, 5 ,6], true),
            TD::Close(true)
        ]), false);
        assert_eq!(w.write(&[0, 1, 2]).unwrap(), 3);
        assert_eq!(w.write(&[3, 4, 5, 6]).unwrap(), 4);
        assert_eq!(w.shutdown().unwrap(), Async::Ready(()));
        w.s.finalize();

        let mut w = AsyncWriteSink::new(TestSink::new(vec![
            TD::StartSend(&[0, 1, 2], true),
            TD::StartSend(&[3, 4, 5 ,6], false),
            TD::PollComplete(true),
            TD::StartSend(&[3, 4, 5 ,6], true),
            TD::Close(true)
        ]), false);
        assert_eq!(w.write(&[0, 1, 2]).unwrap(), 3);
        assert_eq!(w.write(&[3, 4, 5, 6]).unwrap(), 4);
        assert_eq!(w.shutdown().unwrap(), Async::Ready(()));
        w.s.finalize();

        let mut w = AsyncWriteSink::new(TestSink::new(vec![
            TD::StartSend(&[0, 1, 2], true),
            TD::StartSend(&[3, 4, 5], false),
            TD::PollComplete(false),
            TD::PollComplete(false),
            TD::PollComplete(true),
            TD::StartSend(&[3, 4, 5], true),
            TD::StartSend(&[6, 7, 8], true),
            TD::Close(true)
        ]), false);
        assert_eq!(w.write(&[0, 1, 2]).unwrap(), 3);
        assert_eq!(w.write(&[3, 4, 5]).unwrap(), 3);
        assert_eq!(w.write(&[6, 7, 8]).expect_err("EWouldBlock expected!!!").kind(), ErrorKind::WouldBlock);
        assert_eq!(w.write(&[6, 7, 8]).unwrap(), 3);
        assert_eq!(w.shutdown().unwrap(), Async::Ready(()));
        w.s.finalize();

        let mut w = AsyncWriteSink::new(TestSink::new(vec![
            TD::StartSend(&[0, 1, 2], true),
            TD::Close(false),
            TD::Close(true)
        ]), false);
        assert_eq!(w.write(&[0, 1, 2]).unwrap(), 3);
        assert_eq!(w.shutdown().unwrap(), Async::NotReady);
        assert_eq!(w.shutdown().unwrap(), Async::Ready(()));
        w.s.finalize();

    }
}

//======================================================================================================================
// New API
//======================================================================================================================

pub use futures::Async;

enum ChannelState<Connection> {
    Idle,
    Connecting(BFI<Connection>),
    Connected(Connection),
}

#[derive(Debug)]
pub enum CQ<Q, ConnectionData> {
    SetConnection(ConnectionData),
    ClearConnection,
    Send(Q)
}


/// Wraps a Sink into a structure that supports in-band connection management via special messages.
/// `SetConnection(cdata)` establishes connection, `ClearConnection` terminates it.
pub struct Channel<Q, R, Connection, ConnectionData> {
    s: ChannelState<Connection>,
    _q_t: PhantomData<Q>,
    _r_t: PhantomData<R>,
    _connection_data_t: PhantomData<ConnectionData>
}

impl<Q, R, Connection, ConnectionData> Channel<Q, R, Connection, ConnectionData> {
    pub fn new() -> Channel<Q, R, Connection, ConnectionData> {
        Channel {
            s: ChannelState::Idle,
            _q_t: PhantomData,
            _r_t: PhantomData,
            _connection_data_t: PhantomData
        }
    }
}



impl<Q, R, Connection, ConnectionData> Sink for Channel<Q, R, Connection, ConnectionData> where
    Connection: Sink<SinkItem=Q, SinkError=Error>,
    ConnectionData: Into<BFI<Connection>>
{
    type SinkItem = CQ<Q, ConnectionData>;
    type SinkError = Error;

    fn start_send(&mut self, q: CQ<Q, ConnectionData>) -> Result<AsyncSink<CQ<Q, ConnectionData>>> {
        fn start_connecting<Q, Connection, ConnectionData>(cdata: ConnectionData) -> SnV<ChannelState<Connection>, Result<AsyncSink<CQ<Q, ConnectionData>>>>
            where ConnectionData: Into<BFI<Connection>>
        {
            let mut f = cdata.into();
            match f.poll() {
                Ok(Async::Ready(c)) =>
                    SnV::SV(ChannelState::Connected(c), Ok(AsyncSink::Ready)),
                Ok(Async::NotReady) =>
                    SnV::SV(ChannelState::Connecting(f), Ok(AsyncSink::Ready)),
                Err(e) =>
                    SnV::V(Err(e.into()))
            }
        }

        match q {
            CQ::SetConnection(cdata) => switch_state(
                &mut self.s,
                |s| match s {
                    ChannelState::Connected(c) =>
                        match c.close() {
                            Ok(Async::NotReady) =>
                                SnV::V(Ok(AsyncSink::NotReady(CQ::SetConnection(cdata)))),
                            Ok(Async::Ready(())) =>
                                start_connecting(cdata),
                            Err(e) =>
                                SnV::V(Err(e))
                        }
                    _ =>
                        start_connecting(cdata)
                }
            ),
            CQ::ClearConnection => switch_state(
                &mut self.s,
                |s| match s {
                    ChannelState::Connected(c) =>
                        match c.close() {
                            Ok(Async::NotReady) =>
                                SnV::V(Ok(AsyncSink::NotReady(CQ::ClearConnection))),
                            Ok(Async::Ready(())) =>
                                SnV::SV(ChannelState::Idle, Ok(AsyncSink::Ready)),
                            Err(e) =>
                                SnV::V(Err(e))
                        }
                    _ =>
                        SnV::SV(ChannelState::Idle, Ok(AsyncSink::Ready))
                }
            ),
            CQ::Send(q) => switch_state(
                &mut self.s,
                |s| match s {
                    ChannelState::Connected(c) =>
                        match c.start_send(q) {
                            Ok(AsyncSink::NotReady(q)) =>
                                SnV::V(Ok(AsyncSink::NotReady(CQ::Send(q)))),
                            Ok(AsyncSink::Ready) =>
                                SnV::V(Ok(AsyncSink::Ready)),
                            Err(e) =>
                                SnV::V(Err(e))
                        }
                    ChannelState::Connecting(f) =>
                        match f.poll() {
                            Ok(Async::Ready(mut c)) =>
                                match c.start_send(q) {
                                    Ok(AsyncSink::NotReady(q)) =>
                                        SnV::SV(ChannelState::Connected(c), Ok(AsyncSink::NotReady(CQ::Send(q)))),
                                    Ok(AsyncSink::Ready) =>
                                        SnV::SV(ChannelState::Connected(c), Ok(AsyncSink::Ready)),
                                    Err(e) =>
                                        SnV::SV(ChannelState::Connected(c), Err(e))
                                }
                            Ok(Async::NotReady) =>
                                SnV::V(Ok(AsyncSink::NotReady(CQ::Send(q)))),
                            Err(e) =>
                                SnV::V(Err(e.into()))
                        }
                    ChannelState::Idle =>
                        SnV::V(Err(app_error!(other "Channel: Invalid state (Send without SetConnection)")))
                }
            )
        }
    }

    fn poll_complete(&mut self) -> Result<Async<()>> {
        switch_state(
            &mut self.s,
            |s| match s {
                ChannelState::Idle =>
                    SnV::V(Ok(Async::Ready(()))),
                ChannelState::Connecting(f) =>
                    match f.poll() {
                        Ok(Async::Ready(c)) =>
                            SnV::SV(ChannelState::Connected(c), Ok(Async::Ready(()))),
                        Ok(Async::NotReady) =>
                            SnV::V(Ok(Async::NotReady)),
                        Err(e) =>
                            SnV::V(Err(e.into()))
                    }
                ChannelState::Connected(c) =>
                    SnV::V(c.poll_complete())
            }
        )
    }

    fn close(&mut self) -> Result<Async<()>> {
        match &mut self.s {
            ChannelState::Connected(c) =>
                c.close(),
            s@ ChannelState::Connecting(..) => {
                *s = ChannelState::Idle;
                Ok(Async::Ready(()))
            }
            ChannelState::Idle =>
                Ok(Async::Ready(()))
        }
    }
}

fn poll_channel<R, Connection>(s: &mut ChannelState<Connection>) -> Result<Async<Option<R>>> where
    Connection: Stream<Item=R, Error=Error>
{
    switch_state(
        s,
        |s| match s {
            ChannelState::Idle =>
                SnV::V(Err(app_error!(other "Channel: Invalid state (Recv without SetConnection)"))),
            ChannelState::Connecting(f) =>
                match f.poll() {
                    Ok(Async::Ready(mut c)) => {
                        let rv = c.poll();
                        SnV::SV(ChannelState::Connected(c), rv)
                    }
                    Ok(Async::NotReady) =>
                        SnV::V(Ok(Async::NotReady)),
                    Err(e) =>
                        SnV::V(Err(e.into()))
                }
            ChannelState::Connected(c) =>
                SnV::V(c.poll())
        }
    )
}

impl<Q, R, Connection, ConnectionData> Stream for Channel<Q, R, Connection, ConnectionData> where
    Connection: Stream<Item=R, Error=Error>
{
    type Item = R;
    type Error = Error;

    fn poll(&mut self) -> Result<Async<Option<R>>> {
        poll_channel(&mut self.s)
    }
}

pub enum HCQ<ConnectionData> {
    SetConnection(ConnectionData),
    ClearConnection
}

/// Stream-only (half duplex) version of `Channel`
pub struct HChannel<R, Connection, ConnectionData> {
    s: ChannelState<Connection>,
    _r_t: PhantomData<R>,
    _connection_data_t: PhantomData<ConnectionData>
}

impl<R, Connection, ConnectionData> HChannel<R, Connection, ConnectionData> {
    pub fn new() -> HChannel<R, Connection, ConnectionData> {
        HChannel { s: ChannelState::Idle, _r_t: PhantomData, _connection_data_t: PhantomData }
    }
}

impl<R, Connection, ConnectionData> HChannel<R, Connection, ConnectionData> where
    ConnectionData: Into<BFI<Connection>> {
    pub fn connect(cdata: ConnectionData) -> HChannel<R, Connection, ConnectionData> {
        HChannel { s: ChannelState::Connecting(cdata.into()), _r_t: PhantomData, _connection_data_t: PhantomData }
    }
}

impl<R, Connection, ConnectionData> Sink for HChannel<R, Connection, ConnectionData> where
    ConnectionData: Into<BFI<Connection>>
{
    type SinkItem = HCQ<ConnectionData>;
    type SinkError = Error;

    fn start_send(&mut self, q: HCQ<ConnectionData>) -> Result<AsyncSink<HCQ<ConnectionData>>> {
        fn start_connecting<Connection, ConnectionData>(cdata: ConnectionData) -> SnV<ChannelState<Connection>, Result<AsyncSink<HCQ<ConnectionData>>>>
            where ConnectionData: Into<BFI<Connection>>
        {
            let mut f = cdata.into();
            match f.poll() {
                Ok(Async::Ready(c)) =>
                    SnV::SV(ChannelState::Connected(c), Ok(AsyncSink::Ready)),
                Ok(Async::NotReady) =>
                    SnV::SV(ChannelState::Connecting(f), Ok(AsyncSink::Ready)),
                Err(e) =>
                    SnV::V(Err(e.into()))
            }
        }

        match q {
            HCQ::SetConnection(cdata) => switch_state(
                &mut self.s,
                |_| start_connecting(cdata)
            ),
            HCQ::ClearConnection => switch_state(
                &mut self.s,
                |s| SnV::SV(ChannelState::Idle, Ok(AsyncSink::Ready))
            )
        }
    }

    fn poll_complete(&mut self) -> Result<Async<()>> {
        switch_state(
            &mut self.s,
            |s| match s {
                ChannelState::Idle =>
                    SnV::V(Ok(Async::Ready(()))),
                ChannelState::Connecting(f) =>
                    match f.poll() {
                        Ok(Async::Ready(c)) =>
                            SnV::SV(ChannelState::Connected(c), Ok(Async::Ready(()))),
                        Ok(Async::NotReady) =>
                            SnV::V(Ok(Async::NotReady)),
                        Err(e) =>
                            SnV::V(Err(e.into()))
                    }
                ChannelState::Connected(c) =>
                    SnV::V(Ok(Async::Ready(())))
            }
        )
    }

    fn close(&mut self) -> Result<Async<()>> {
        self.s = ChannelState::Idle;
        Ok(Async::Ready(()))
    }
}

impl<R, Connection, ConnectionData> Stream for HChannel<R, Connection, ConnectionData> where
    Connection: Stream<Item=R, Error=Error>
{
    type Item = R;
    type Error = Error;

    fn poll(&mut self) -> Result<Async<Option<R>>> {
        poll_channel(&mut self.s)
    }
}


use std::time::Instant;
pub use std::time::Duration;

/// Channel with automatic connection management and connection idle timeout.
/// Idle timer is restarted on each successful poll_complete preceded by a successful start_send,
/// and checked upon entry in start_send (where the connection is restarted if expired),
/// poll_complete and poll (two latter fail if expired). The idle timer is also restarted upon
/// starting connection.
pub struct AutoChannel<Q, R, Connection, ConnectionData> {
    c: Channel<Q, R, Connection, ConnectionData>,
    timeout_point: Instant,
    send_started: bool,
    timeout: Duration,
    cdata: ConnectionData
}

impl<Q, R, Connection, ConnectionData> AutoChannel<Q, R, Connection, ConnectionData> where
    Connection: Sink<SinkItem=Q, SinkError=Error>,
    ConnectionData: Into<BFI<Connection>> + Clone
{
    pub fn new(cdata: ConnectionData, timeout: Duration) -> AutoChannel<Q, R, Connection, ConnectionData> {
        AutoChannel {
            c: Channel::new(),
            timeout_point: Instant::now() - timeout,    //a point in the past
            send_started: false,
            timeout,
            cdata
        }
    }

    /// returns true if ok to start sending, false if the channel is busy
    /// (e.g. tearing down previous connection)
    fn check_connection(&mut self, fail_on_timeout: bool) -> Result<bool> {
        let now = Instant::now();
        if now >= self.timeout_point {
            if fail_on_timeout {
                Err(app_error!(other "AutoChannel: connection idle timeout"))
            } else {
                match self.c.start_send(CQ::SetConnection(self.cdata.clone()))? {
                    AsyncSink::Ready => {
                        self.connection_activity();
                        self.send_started = false;
                        Ok(true)
                    },
                    AsyncSink::NotReady(_) =>
                        Ok(false)
                }
            }
        } else {
            Ok(true)
        }
    }

    /// Schedules idle timer from now
    fn connection_activity(&mut self) {
        self.timeout_point = Instant::now() + self.timeout;
    }
}

impl<Q, R, Connection, ConnectionData> Sink for AutoChannel<Q, R, Connection, ConnectionData> where
    Connection: Sink<SinkItem=Q, SinkError=Error>,
    ConnectionData: Into<BFI<Connection>> + Clone + Debug,
    Q: Debug
{
    type SinkItem = Q;
    type SinkError = Error;

    fn start_send(&mut self, item: Q) -> Result<AsyncSink<Q>> {
        if self.check_connection(false)? {
            match self.c.start_send(CQ::Send(item))? {
                AsyncSink::NotReady(CQ::Send(q)) =>
                    Ok(AsyncSink::NotReady(q)),
                AsyncSink::Ready => {
                    self.send_started = true;
                    Ok(AsyncSink::Ready)
                }
                other =>
                    Err(app_error!(other "AutoChannel: Unexpected return value from start_send: {:?}", other))
            }
        } else {
            Ok(AsyncSink::NotReady(item))
        }
    }

    fn poll_complete(&mut self) -> Result<Async<()>> {
        let _ = self.check_connection(true)?;
        let rv = self.c.poll_complete()?;
        if self.send_started && rv.is_ready() {
            self.send_started = false;
            self.connection_activity()
        }
        Ok(rv)
    }

    fn close(&mut self) -> Result<Async<()>> {
        if self.send_started {
            let _ = self.check_connection(true)?;
            self.send_started = false;
        }
        self.c.close()
    }
}

impl<Q, R, Connection, ConnectionData> Stream for AutoChannel<Q, R, Connection, ConnectionData> where
    Connection: Stream<Item=R, Error=Error> + Sink<SinkItem=Q, SinkError=Error>,
    ConnectionData: Into<BFI<Connection>> + Clone
{
    type Item = R;
    type Error = Error;

    fn poll(&mut self) -> Result<Async<Option<R>>> {
        let _ = self.check_connection(true)?;
        self.c.poll()
    }
}



//----------------------------------------------------------------------------------------------------------------------
#[derive(Debug)]
enum AtomicSendReq<LQ> {
    Enqueue(LQ),
    Flush,
    NOP
}

#[derive(Debug)]
pub enum SendReq<LQ> {
    EnqueueAndFlush(LQ),
    Enqueue(LQ),
    Flush,
    NOP
}

/// Non-trivial state values for `SendState`
#[derive(Debug)]
enum SendStateE<LQ> {
    FSF(LQ),
    FS(LQ),
    SF(LQ),
    S(LQ),
    F
}

/// Sending state. A queue-like object. pop (take) and pull(untake) are at front, populate is push at back
#[derive(Debug)]
struct SendState<LQ> {
    s: Option<SendStateE<LQ>>
}

impl<LQ: Debug> SendState<LQ> {
    fn new() -> SendState<LQ> { SendState { s: None } }
    fn is_empty(&self) -> bool { self.s.is_none() }
    fn state_id(&self) -> Option<std::mem::Discriminant<SendStateE<LQ>>> {
        self.s.as_ref().map(|e| std::mem::discriminant(e))
    }
    fn take(&mut self) -> AtomicSendReq<LQ> {
        use self::SendStateE as S;
        use self::AtomicSendReq as A;
        switch_state(&mut self.s, |s| match s.take() {
            Some(S::FSF(q)) => SnV::SV(Some(S::SF(q)), A::Flush),
            Some(S::FS(q)) => SnV::SV(Some(S::S(q)), A::Flush),
            Some(S::SF(q)) => SnV::SV(Some(S::F), A::Enqueue(q)),
            Some(S::S(q)) => SnV::SV(None, A::Enqueue(q)),
            Some(S::F) => SnV::SV(None, A::Flush),
            None => SnV::SV(None, A::NOP)
        })
    }
    fn untake(&mut self, r: AtomicSendReq<LQ>) -> Result<()> {
        use self::SendStateE as S;
        use self::AtomicSendReq as A;
        switch_state(&mut self.s, move |s| match (s.take(), r) {
            (Some(S::SF(q)), A::Flush) =>  SnV::SV(Some(S::FSF(q)), Ok(())),
            (Some(S::S(q)), A::Flush) => SnV::SV(Some(S::FS(q)), Ok(())),
            (Some(S::F), A::Enqueue(q)) => SnV::SV(Some(S::SF(q)), Ok(())),
            (None, A::Enqueue(q))  => SnV::SV(Some(S::S(q)), Ok(())),
            (None, A::Flush) => SnV::SV(Some(S::F), Ok(())),
            (None, A::NOP)  => SnV::SV(None, Ok(())),
            (s, a) => SnV::SV(None, Err(app_error!(other "SendState: untake: Invalid s/a {:?}/{:?}", s, a)))
        })
    }
    fn populate(&mut self, p: SendReq<LQ>) -> Result<()> {
        use self::SendStateE as S;
        use self::SendReq as A;
        switch_state(&mut self.s, move |s| match (s.take(), p) {
            (Some(S::S(q)), A::Flush) => SnV::SV(Some(S::SF(q)), Ok(())),
            (Some(S::F), A::Enqueue(q)) => SnV::SV(Some(S::FS(q)), Ok(())),
            (Some(S::F), A::EnqueueAndFlush(q)) => SnV::SV(Some(S::FSF(q)), Ok(())),
            (None, A::Enqueue(q))  => SnV::SV(Some(S::S(q)), Ok(())),
            (None, A::EnqueueAndFlush(q))  => SnV::SV(Some(S::SF(q)), Ok(())),
            (None, A::Flush) => SnV::SV(Some(S::F), Ok(())),
            (None, A::NOP)  => SnV::SV(None, Ok(())),
            (s, a) => SnV::SV(None, Err(app_error!(other "SendState: populate: Invalid s/a {:?}/{:?}", s, a)))
        })
    }
}


/// Behaviour trait for `StreamProtocol`
pub trait StreamProtocolFsm {
    /// Upper-layer `Stream` item type
    type R: Debug;
    /// Lower-layer `Sink` item type
    type LQ: Debug;
    /// Lower-layer `Stream` item type
    type LR: Debug;
    /// Obtain next item to send along with flush flag
    fn get_downstream(&mut self) -> Result<SendReq<Self::LQ>>;
    /// handle a message from the lower layer
    fn handle_upstream(&mut self, lr: Option<Self::LR>) -> Result<Async<Option<Self::R>>>;
}

/// Protoclol layer adapter that converts lower-layer `Sink`+`Stream` into upper-layer `Stream`.
pub struct StreamProtocol<L, FSM> where FSM: StreamProtocolFsm {
    s: SendState<FSM::LQ>,
    l: L,
    fsm: FSM
}

impl <L, FSM> StreamProtocol<L, FSM> where
    FSM: StreamProtocolFsm {
    pub fn new(l: L, fsm: FSM) -> StreamProtocol<L, FSM> {
        StreamProtocol {
            s: SendState::new(),
            l,
            fsm
        }
    }
    pub fn into_parts(self) -> (L, FSM) {
        (self.l, self.fsm)
    }
}

impl <L, FSM> StreamProtocol<L, FSM> where
    L: Sink<SinkItem=FSM::LQ, SinkError=Error> + Stream<Item=FSM::LR, Error=Error>,
    FSM: StreamProtocolFsm {

    fn push_out(&mut self) -> Result<()> {
        use self::AtomicSendReq as A;
        loop {
            if self.s.is_empty() {
                self.s.populate(self.fsm.get_downstream()?)?;
                trace!("get_downstream => {:?}", self.s)
            }
            match self.s.take() {
                A::Enqueue(lq) =>
                    if let AsyncSink::NotReady(lq) = self.l.start_send(lq)? {
                        self.s.untake(A::Enqueue(lq))?;
                        self.s.untake(A::Flush)?
                    }
                A::Flush =>
                    if let Async::NotReady = self.l.poll_complete()? {
                        self.s.untake(A::Flush)?;
                        //stuck in flushing, so give up this time
                        break Ok(())
                    }
                A::NOP =>
                    //nothing to send out this time
                    break Ok(())
            }
        }
    }
}

impl<L, FSM> Stream for StreamProtocol<L, FSM> where
    L: Sink<SinkItem=FSM::LQ, SinkError=Error> + Stream<Item=FSM::LR, Error=Error>,
    FSM: StreamProtocolFsm
{
    type Item = FSM::R;
    type Error = Error;

    fn poll(&mut self) -> Result<Async<Option<FSM::R>>> {
        self.push_out()?;
        loop {

            match self.l.poll()? {
                Async::Ready(m) => {
                    trace!("handle_upstream <= {:?}", m);
                    let w = self.fsm.handle_upstream(m);
                    trace!("handle_upstream => {:?}", w);
                    let w = w?;
                    self.push_out()?;
                    if let Async::Ready(w) = w {
                        break Ok(Async::Ready(w))
                    }
                }
                Async::NotReady =>
                    break Ok(Async::NotReady)
            }
        }
    }
}
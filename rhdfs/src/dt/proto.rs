//! Datatransfer protocol upper level

use std::io::ErrorKind;
use std::net::SocketAddr;
use std::fmt::Debug;

use futures::{Future, Stream, Poll, Sink, Async, AsyncSink};
use futures::future::{ok, err};

use tokio_io::AsyncRead;
use tokio_io::codec::Framed;
use tokio_tcp::TcpStream;

pub use log::Level::Trace as LogTrace;
use super::codec::{DtCodec, DtReq, DtRsp};
use *;


pub struct Connection {
    io: Framed<TcpStream, DtCodec>,
    client_name: String
}

impl Stream for Connection {
    type Item = <Framed<TcpStream, DtCodec> as Stream>::Item;
    type Error = <Framed<TcpStream, DtCodec> as Stream>::Error;

    fn poll(&mut self) -> Poll<Option<Self::Item>, Self::Error> {
        self.io.poll()
    }
}

/// 'Action' part of the output of a protocol FSM event handler
#[derive(Debug)]
pub enum ProtocolFsmResult {
    /// Send out the message
    Send(DtReq),
    /// Send out the message and wait for an incoming message (full duplex mode)
    SendAndWait(DtReq),
    /// Wait until a message arrives from the remote end
    Wait,
    /// Exit the IO loop with success
    ReturnSuccess,
    /// Exit the IO Loop with error
    ReturnError(Error)
}

/// The macro for various protocol FSM event handler output flavors
#[macro_export]
macro_rules! pfsm {
    {wait / goto $s:expr} => { (ProtocolFsmResult::Wait, $s) };
    {send ($r:expr) / goto $s:expr} => { (ProtocolFsmResult::Send($r), $s) };
    {return success / goto $s:expr} => { (ProtocolFsmResult::ReturnSuccess, $s) };
    {return error ($e:expr) / goto $s:expr} => { (ProtocolFsmResult::ReturnError($e), $s) };
}



#[macro_export]
macro_rules! pfsm_trace {
    {$s:expr, $e:expr, $r:expr} => {
        if log_enabled!(target: "protocol_fsm", LogTrace) {
            let h = format!("{:?}/{:?}", $s, $e);
            let result = $r;
            trace!(target: "protocol_fsm", "{} => {:?}/{:?}", h, result.0, result.1);
            result
        } else {
            $r
        }
     };
}

/// Protocol FSM event handler
pub trait ProtocolFsm : Debug {
    /// Idle event occurs a) upon start or b) upon successful outgoing packet transmission
    fn idle(self) -> (ProtocolFsmResult, Self);
    /// The event is raised upon successful incoming packet reception
    fn incoming(self, rsp: DtRsp) -> (ProtocolFsmResult, Self);
}

impl Connection {
    pub fn connect(addr: &SocketAddr, client_name: String) -> BFI<Connection> {
        trace!("Trying to connect to {}, client_name={}", addr, client_name);
        let rv =
            TcpStream::connect(addr)
                .map(|c| Connection { io: c.framed(DtCodec::new()), client_name });
        Box::new(rv)
    }

    #[inline]
    fn broken_pipe_error() -> IoError {
        IoError::new(ErrorKind::BrokenPipe, app_error!(other "broken pipe"))
    }

    fn send_req(self, mut request: DtReq) -> BFI<Connection> {
        let client_name = self.client_name;
        request.set_client_name(&client_name);
        Box::new(self.io.send(request).map(|c| Connection { io: c, client_name }))
    }

    fn get_resp(self) -> BFI<(DtRsp, Connection)> {
        let rv =
            self.into_future().and_then(|(orsp, c)|
                match orsp {
                    Some(r) => ok((r, c)),
                    None => err((Connection::broken_pipe_error(), c))
                }
            ).map_err(|(e, _)| e);
        Box::new(rv)
    }

    #[inline]
    fn call(self, request: DtReq) -> BFI<(DtRsp, Connection)> {
        Box::new(self.send_req(request).and_then(|c| c.get_resp()))
    }

    #[inline]
    pub fn run<P>(self, p: P) -> BF<(Connection, P), (Error, P)>
        where P: ProtocolFsm + Send +'static {
        //Box::new(FsmRunner::new(self, p))
        Box::new(FdxFsmRunner::new(self, p))
    }
}

enum FsmRunner<P> {
    Init(Connection, P),
    Send(P, BFI<Connection>),
    Wait(P, BFI<(DtRsp, Connection)>),
    Null
}

type FRR<P> = StdResult<Async<(Connection, P)>, (Error, P)>;

impl<P> Future for FsmRunner<P>
    where P: ProtocolFsm + Send +'static {
    type Item = (Connection, P);
    type Error = (Error, P);

    fn poll(&mut self) -> FRR<P> {
        #[inline]
        fn absolutely_not_ready<P>(s: FsmRunner<P>) -> (FsmRunner<P>, Option<FRR<P>>) { (s, None)  }

        #[inline]
        fn not_ready<P>(s: FsmRunner<P>) -> (FsmRunner<P>, Option<FRR<P>>) { (s, Some(Ok(Async::NotReady)))  }

        #[inline]
        fn ready<P>(v: FRR<P>) -> (FsmRunner<P>, Option<FRR<P>>) { (FsmRunner::Null, Some(v)) }

        fn process_result<P>(c: Connection, (r, p): (ProtocolFsmResult, P)) -> (FsmRunner<P>, Option<FRR<P>>) {
            match r {
                ProtocolFsmResult::Send(req) =>
                    absolutely_not_ready(FsmRunner::Send(p, c.send_req(req))),
                ProtocolFsmResult::SendAndWait(..) =>
                    panic!("FsmRunner: SendAndWait not supported"),
                ProtocolFsmResult::Wait =>
                    absolutely_not_ready(FsmRunner::Wait(p, c.get_resp())),
                ProtocolFsmResult::ReturnSuccess =>
                    ready(Ok(Async::Ready((c, p)))),
                ProtocolFsmResult::ReturnError(e) =>
                    ready(Err((e.into(), p)))
            }
        }

        loop {
            let (s1, rv) = match std::mem::replace(self, FsmRunner::Null) {
                FsmRunner::Init(c, p) => process_result(c, p.idle()),
                FsmRunner::Send(p, mut conn_f) => match conn_f.poll() {
                    Ok(Async::NotReady) =>
                        not_ready(FsmRunner::Send(p, conn_f)),
                    Ok(Async::Ready(c)) =>
                        process_result(c, p.idle()),
                    Err(e) =>
                        ready(Err((e.into(), p)))
                }
                FsmRunner::Wait(p, mut rsp_f) => match rsp_f.poll() {
                    Ok(Async::NotReady) =>
                        not_ready(FsmRunner::Wait(p, rsp_f)),
                    Ok(Async::Ready((rsp, c))) =>
                        process_result(c, p.incoming(rsp)),
                    Err(e) =>
                        ready(Err((e.into(), p)))
                }
                FsmRunner::Null =>
                    panic!("Unfused call to completed FsmRunner"),
            };
            *self = s1;
            if let Some(v) = rv { break v }
        }
    }
}

impl<P> FsmRunner<P>
    where P: ProtocolFsm + Send +'static {

    fn new(c: Connection, p: P) -> FsmRunner<P> { FsmRunner::Init(c, p) }
}
//--------------------------------------------------------------------------------------------------
#[derive(Debug)]
enum FsmEvent {
    Idle,
    Incoming(DtRsp),
    Err(Error),
    Success,
    EndOfStream,
    NOP
}

/// Full-duplex FSM runner data
struct FdxFsmRunnerData<P> {
    c: Connection,
    p: P,
    sending: bool,
    receiving: bool
}

impl<P> FdxFsmRunnerData<P> where P: ProtocolFsm + Send +'static {
    #[inline]
    fn new(c: Connection, p: P) -> FdxFsmRunnerData<P> {
        FdxFsmRunnerData { c, p, sending: false, receiving: false }
    }

    #[inline]
    fn start_send(mut self, mut m: DtReq, receive: bool) -> StdResult<Self, (Error, P)> {
        if !self.sending {
            m.set_client_name(&self.c.client_name);
            match self.c.io.start_send(m) {
                Ok(AsyncSink::Ready) =>
                    Ok(Self { sending: true, receiving: receive, ..self }),
                Ok(AsyncSink::NotReady(w)) =>
                    Err((app_error!(other "Overlapped sending on {:?}", w), self.p)),
                Err(e) =>
                    Err((e.into(), self.p))
            }
        } else {
            Err((app_error!(other "Overlapped sending"), self.p))
        }
    }

    #[inline]
    fn start_recv(self) -> StdResult<Self, (Error, P)> {
        Ok(Self { receiving: true, ..self })
    }

    #[inline]
    fn into_success(self) -> (Option<Self>, FRR<P>) { (None, Ok(Async::Ready((self.c, self.p)))) }
    #[inline]
    fn into_nop(self) -> (Option<Self>, FRR<P>) { (Some(self), Ok(Async::NotReady)) }
    #[inline]
    fn into_error(self, e: Error) -> (Option<Self>, FRR<P>) { (None, Err((e, self.p))) }

    fn poll_send(&mut self) -> FsmEvent {
        if self.sending {
            match self.c.io.poll_complete() {
                Ok(Async::Ready(())) => {
                    self.sending = false;
                    FsmEvent::Idle
                }
                Ok(Async::NotReady) =>
                    FsmEvent::NOP,
                Err(e) =>
                    FsmEvent::Err(e.into())
            }
        } else {
            FsmEvent::NOP
        }
    }

    fn poll_send_byval(mut self) -> StdResult<(Self, FsmEvent), (Error, P)> {
        let ev = self.poll_send();
        Ok((self, ev))
    }

    fn poll_recv(&mut self) -> FsmEvent {
        if self.receiving {
            match self.c.io.poll() {
                Ok(Async::Ready(Some(v))) => {
                    self.receiving = false;
                    FsmEvent::Incoming(v)
                }
                Ok(Async::Ready(None)) => {
                    self.receiving = false;
                    FsmEvent::EndOfStream
                }
                Ok(Async::NotReady) =>
                    FsmEvent::NOP,
                Err(e) =>
                    FsmEvent::Err(e.into())
            }
        } else {
            FsmEvent::NOP
        }
    }

    fn poll_recv_byval(mut self) -> StdResult<(Self, FsmEvent), (Error, P)> {
        let ev = self.poll_recv();
        Ok((self, ev))
    }

    fn handle_event_int(mut self, ev: FsmEvent) -> StdResult<(Self, FsmEvent), (Error, P)> {
        let (r, p1) = match ev {
            FsmEvent::Idle =>
                self.p.idle(),
            FsmEvent::Incoming(m) =>
                self.p.incoming(m),
            FsmEvent::NOP | FsmEvent::EndOfStream | FsmEvent::Success | FsmEvent::Err(..) =>
                unreachable!()
        };
        self.p = p1;
        match r {
            ProtocolFsmResult::ReturnSuccess =>
                Ok((self, FsmEvent::Success)),
            ProtocolFsmResult::ReturnError(e) =>
                Err((e, self.p)),
            ProtocolFsmResult::Send(m) =>
                self.start_send(m, false).and_then(|s| s.poll_send_byval()),
            ProtocolFsmResult::SendAndWait(m) =>
                self.start_send(m, true).and_then(|s| s.poll_send_byval()),
            ProtocolFsmResult::Wait =>
                self.start_recv().and_then(|s| s.poll_recv_byval()),
        }
    }

    fn handle_event(mut self, mut ev: FsmEvent) -> (Option<Self>, FRR<P>) {
        let mut r = Ok((self, ev));
        loop {
            match r {
                Ok((s, FsmEvent::NOP)) =>
                    break s.into_nop(),
                Ok((s, FsmEvent::Success)) =>
                    break s.into_success(),
                Ok((s, FsmEvent::Err(e))) =>
                    break s.into_error(e),
                Ok((s, FsmEvent::EndOfStream)) =>
                    break s.into_error(app_error!(other "dt: Premature end of input stream")),
                Ok((s, ev)) =>
                    r = s.handle_event_int(ev),
                Err((e, p)) =>
                    break (None, Err((e, p)))
            }
        }
    }
}

/// Full-duplex FSM runner
enum FdxFsmRunner<P> {
    Init(Connection, P),
    Active(FdxFsmRunnerData<P>),
    Null
}

impl<P> FdxFsmRunner<P> where P: ProtocolFsm + Send +'static {
    fn new(c: Connection, p: P) -> FdxFsmRunner<P> {
        FdxFsmRunner::Init(c, p)
    }

    fn handle_event(&mut self, ev: FsmEvent) -> FRR<P> {
        if let FsmEvent::NOP = ev {
            Ok(Async::NotReady)
        } else {
            let (s1, rv) = match std::mem::replace(self, FdxFsmRunner::Null) {
                FdxFsmRunner::Init(c, p) =>
                    FdxFsmRunnerData::new(c, p).handle_event(FsmEvent::Idle),
                FdxFsmRunner::Active(d) =>
                    d.handle_event(ev),
                FdxFsmRunner::Null =>
                    panic!("invalid handle_event usage")
            };
            *self = s1.map_or(FdxFsmRunner::Null, |d| FdxFsmRunner::Active(d));
            rv
        }
    }
}

impl<P> Future for FdxFsmRunner<P>
    where P: ProtocolFsm + Send +'static {
    type Item = (Connection, P);
    type Error = (Error, P);

    fn poll(&mut self) -> FRR<P> {
        let (e_recv, e_send) = match self {
            FdxFsmRunner::Init(c, p) =>
                (FsmEvent::Idle, FsmEvent::NOP),
            FdxFsmRunner::Active(d) => {
                let e_recv = d.poll_recv();
                let e_send = d.poll_send();
                (e_recv, e_send)
            }
            FdxFsmRunner::Null =>
                panic!("Invalid future state: Null")
        };

        let r_recv = self.handle_event(e_recv);
        if let Ok(Async::NotReady) = r_recv {
            self.handle_event(e_send)
        } else {
            r_recv
        }
    }
}
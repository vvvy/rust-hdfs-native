//! Datatransfer protocol upper level

use std::io::ErrorKind;
use std::net::SocketAddr;
use std::fmt::Debug;

use futures::{Future, Stream, Poll, Sink, Async};
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
        Box::new(FsmRunner::new(self, p))
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

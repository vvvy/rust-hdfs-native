//! Datatransfer protocol upper level

use std::io::ErrorKind;
use std::net::SocketAddr;
use std::fmt::Debug;

use futures::{Future, Stream, Poll, Sink};
use futures::future::{ok, err};

use tokio_io::AsyncRead;
use tokio_io::codec::Framed;
use tokio_tcp::TcpStream;

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
    /// Exit the IO loop
    Exit
}

/// The macro for various protocol FSM event handler output flavors
#[macro_export]
macro_rules! pfsm {
    {wait / goto $s:expr} => { (ProtocolFsmResult::Wait, $s) };
    {send ($r:expr) / goto $s:expr} => { (ProtocolFsmResult::Send($r), $s) };
    {exit $s:expr} => { (ProtocolFsmResult::Exit, $s) };
}

pub use log::Level::Trace as LogTrace;

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
pub trait ProtocolFsm : Debug + ErrorAccumulator {
    /// Idle event occurs a) upon start or b) upon successful outgoing packet transmission
    fn idle(self) -> (ProtocolFsmResult, Self);
    /// The event is raised upon successful incoming packet reception
    fn incoming(self, rsp: DtRsp) -> (ProtocolFsmResult, Self);
}

impl Connection {
    pub fn connect(addr: &SocketAddr, client_name: String) -> BFI<Connection> {
        let rv =
            TcpStream::connect(addr)
                .map(|c| Connection { io: c.framed(DtCodec::new()), client_name });
        Box::new(rv)
    }

    //#[inline]
    //pub fn client_name(&self) -> &str { &self.client_name }

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

    /*
    fn get_resp_recur<P>(self, mut p: P) -> Box<Future<Item=(Connection, P), Error=(IoError, Connection)> + Send>
        where P: FnMut(DtRsp) -> IoResult<bool> + Send + 'static{
        let rv =
            self.into_future().and_then(|(orsp, c)|
                match orsp {
                    Some(r) => match p(r) {
                        Ok(true) => c.get_resp_recur(p),
                        Ok(false) => Box::new(ok((c, p))),
                        Err(e) => Box::new(err((e, c)))
                    },
                    None => Box::new(err((Connection::broken_pipe_error(), c)))
                }
            );
        Box::new(rv)
    }
*/
    #[inline]
    fn call(self, request: DtReq) -> BFI<(DtRsp, Connection)> {
        Box::new(self.send_req(request).and_then(|c| c.get_resp()))
    }

    #[inline]
    pub fn run<P>(self, p: P) -> BF<(Connection, P), P>
        where P: ProtocolFsm + Send +'static {
        self.fsm_idle(p)
    }

    #[inline]
    fn fsm_idle<P>(self, p: P) -> BF<(Connection, P), P>
        where P: ProtocolFsm + Send +'static {
        self.fsm_result(pfsm_trace!(p, "<idle>", p.idle()))
    }

    #[inline]
    fn fsm_incoming<P>(self, p: P, rsp: DtRsp) -> BF<(Connection, P), P>
        where P: ProtocolFsm + Send +'static {
        self.fsm_result(pfsm_trace!(p, rsp, p.incoming(rsp)))
    }

    fn fsm_result<P>(self, (r, p): (ProtocolFsmResult, P)) -> BF<(Connection, P), P>
        where P: ProtocolFsm + Send +'static {
        match r {
            ProtocolFsmResult::Send(req) =>
                Box::new(self
                             .send_req(req)
                             .then(|w| match w {
                                 Ok(c) => c.fsm_idle(p),
                                 Err(e) => Box::new(err(p.error(e)))
                             })
                ),
            ProtocolFsmResult::Wait =>
                self.fsm_wait(p),
            ProtocolFsmResult::Exit =>
                Box::new(ok((self, p)))
        }
    }

    fn fsm_wait<P>(self, p: P) -> BF<(Connection, P), P>
        where P: ProtocolFsm + Send +'static {
        let rv =
            self
                .into_future()
                .then(|w| match w {
                    Ok((or, c)) => match or {
                        Some(r) => c.fsm_incoming(p, r),
                        None => Box::new(err(p.error(Connection::broken_pipe_error())))
                    }
                    Err((e, _c)) => Box::new(err(p.error(e)))
                });
        Box::new(rv)
    }
}


//! ClientNamenodeProtocol RPC definitions

use std::net::{SocketAddr/*, Shutdown*/};

use futures::Future;
use tokio_core::net::{TcpStream, TcpStreamNew};
use tokio_core::reactor::Core;
use tokio_io;
use byteorder::{ByteOrder, BigEndian};
use ::Result;
use util;
use codec_nn::{handshake_packet, request_packet, response_packet};

use protobuf::Message;

/// Hand shake with a namenode. Note that no response in supposed from the server on success.
fn handshake(core: &mut Core, socket: TcpStreamNew, pkt: Vec<u8>) -> Result<TcpStream> {
    let request =
        socket.and_then(|socket| {
            trace!("[hs]< {:?}", pkt);
            tokio_io::io::write_all(socket, pkt)
        });

    let (socket, _) = core.run(request)?;
    trace!("[hs]>");
    Ok(socket)
}

/// Send a request packet (pkt) and receive a response from the namenode
fn call(core: &mut Core, socket: TcpStream, pkt: Vec<u8>) -> Result<(TcpStream, Vec<u8>)> {
    trace!("< {:?}", pkt);
    let mut pkt_len = [0u8; 4];
    BigEndian::write_u32(&mut pkt_len, pkt.len() as u32);

    let request =
        tokio_io::io::write_all(socket, &pkt_len)
            .and_then(|(socket, _) |tokio_io::io::write_all(socket, pkt));

    let response =
        request.and_then(|(socket, _request)| {
            tokio_io::io::read_exact(socket, [0u8; 4])
                .and_then(|(socket, b)| {
                    let packet_len = BigEndian::read_u32(&b);
                    let r  = util::vector_of_size(packet_len as usize);
                    tokio_io::io::read_exact(socket, r)
                })
        });

    let (socket, data) = core.run(response)?;
    trace!("> {:?}", data);
    Ok((socket, data))
}

/// Immutable connection data
pub struct Connection {
    client_id: Vec<u8>,
    effective_user: String
}

/// Mutable and pass-through connection data
pub struct ConnectionState {
    socket: TcpStream,
    call_id: i32,
    core: Core
}

impl Connection {
    pub fn new(client_id: Vec<u8>, effective_user: String) -> Connection {
        Connection { /*client_id:*/ client_id, /*effective_user:*/ effective_user }
    }

    pub fn connect(&self, mut core: Core, addr: SocketAddr) -> Result<ConnectionState> {
        let handle = core.handle();
        let socket = TcpStream::connect(&addr, &handle);
        let socket_c = handshake(&mut core, socket, handshake_packet(
            self.client_id.clone(),
            self.effective_user.clone()
        )?)?;
        Ok(ConnectionState { socket: socket_c, call_id: 0, core })
    }

    pub fn call<Q: Message, R: Message + Default>(
        &self,
        state: ConnectionState,
        request_name: String,
        req: Q) -> Result<(ConnectionState, R)>
    {
        let ConnectionState { socket, call_id, mut core } = state;

        let request_payload = request_packet(
            self.client_id.clone(),
            call_id,
            request_name,
            req)?;

        let (socket, response_payload) = { call(&mut core, socket, request_payload)? };

        let result = response_packet(response_payload, R::default())?;
        Ok((ConnectionState { socket, call_id: call_id + 1, core }, result))
    }

    /*pub fn disconnect(state: ConnectionState) -> Result<()> {
        let ConnectionState { socket, call_id: _, core: _ } = state;
        socket.shutdown(Shutdown::Both)?;
        Ok(())
    }*/
}
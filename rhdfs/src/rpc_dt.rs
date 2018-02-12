//! DatatransferProtocol RPC definitions

use std::net::{SocketAddr/*, Shutdown*/};

use futures::Future;
use tokio_core::net::{TcpStream, TcpStreamNew};
use tokio_core::reactor::Core;
use tokio_io;
use byteorder::{ByteOrder, BigEndian};
use ::Result;
use util;
//use codec_nn::{handshake_packet, request_packet, response_packet};

use protobuf::Message;

const DATA_TRANSFER_VERSION: u16 = 28;

/// Send a request packet (pkt) and receive a response
fn call(core: &mut Core, socket: TcpStream, opcode: u8, pkt: Vec<u8>) -> Result<(TcpStream, Vec<u8>)> {
    trace!("< {:?}", pkt);

    // A op request to a datanode:
    // +-----------------------------------------------------------+
    // |  Data Transfer Protocol Version, int16                    |
    // +-----------------------------------------------------------+
    // |  Op code, 1 byte                                          |
    // +-----------------------------------------------------------+
    // |  Op-specific payload, variable length                     |
    // +-----------------------------------------------------------+
    // See: org.apache.hadoop.hdfs.protocol.datatransfer.Receiver.readOp()
    let mut frame_header = [0u8; 3];
    BigEndian::write_u16(&mut frame_header[0..2], DATA_TRANSFER_VERSION);
    frame_header[2] = opcode;

    let request =
        tokio_io::io::write_all(socket, &frame_header)
            .and_then(|(socket, _) |tokio_io::io::write_all(socket, pkt));


    // The initial response from a datanode, in the case of reads and writes:
    // +-----------------------------------------------------------+
    // |  varint length + BlockOpResponseProto                     |
    // +-----------------------------------------------------------+

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

/// Receive a stream chunk.
/// "When serialized, this header is written out as a protocol buffer, preceded by a 4-byte integer
/// representing the full packet length, and a 2-byte short representing the header length."
fn recv_chunk(core: &mut Core, socket: TcpStream) -> Result<(TcpStream, (Vec<u8>, Vec<u8>))> {
    // TODO check out if it is possible that packet_len == 0
    let input = tokio_io::io::read_exact(socket, [0u8; 6])
        .and_then(|(socket, b)| {
            let packet_len = BigEndian::read_u32(&b[0..4]);
            let header_len = BigEndian::read_u16(&b[4..6]);
            assert!(header_len as u32 + 2 <= packet_len);
            let header = util::vector_of_size(header_len as usize);
            let data = util::vector_of_size((packet_len - 2 - header_len as u32) as usize);
            tokio_io::io::read_exact(socket, header)
                .and_then(|(socket, header)|
                    tokio_io::io::read_exact(socket, data).map(|(socket, data)| (socket, (header, data)))
                )
        });
    let (socket, (header, data)) = core.run(input)?;
    trace!("> {:?}", data);
    Ok((socket, (header, data)))
}
/*

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
}
*/
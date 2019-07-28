#[macro_use]
extern crate futures;
extern crate tokio;

use std::error;
use std::io;
use std::net::SocketAddr;

use protobuf::parse_from_bytes;

use tokio::net::UdpSocket;
use tokio::prelude::*;

use sidecar::events::EventBundle;
use sidecar::metrics::handle_event_bundle;

struct Server {
    socket: UdpSocket,
    buffer: Vec<u8>,
}

impl Future for Server {
    type Item = ();
    type Error = io::Error;

    fn poll(&mut self) -> Poll<(), io::Error> {
        loop {
            let read_byte_count = try_ready!(self.socket.poll_recv(&mut self.buffer));
            if read_byte_count > 0 {
                let bundle_res = parse_from_bytes::<EventBundle>(&self.buffer[0..read_byte_count]);
                match bundle_res {
                    Ok(bundle) => handle_event_bundle(&bundle),
                    Err(err) => println!("Got invalid bundle input:\n  error: {:?}", err),
                }
            }
        }
    }
}

fn main() -> Result<(), Box<error::Error>> {
    let addr = "127.0.0.1:4000".to_string().parse::<SocketAddr>()?;
    let socket = UdpSocket::bind(&addr)?;
    println!("Listening on: {}", socket.local_addr()?);

    let server = Server {
        socket: socket,
        buffer: vec![0; 1024],
    };

    tokio::run(server.map_err(|e| println!("server error = {:?}", e)));
    Ok(())
}

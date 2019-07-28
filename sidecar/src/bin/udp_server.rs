#[macro_use]
extern crate futures;
extern crate tokio;

use std::error;
use std::io;
use std::net::SocketAddr;

// use protobuf::CodedInputStream;
// use protobuf::Message;
use protobuf::parse_from_bytes;

use tokio::net::UdpSocket;
use tokio::prelude::*;

use sidecar::hello::GreetingList;

struct Server {
    socket: UdpSocket,
    buf: Vec<u8>,
    // to_send: Option<(usize, SocketAddr)>,
}

// impl Future for Server {
//     type Item = ();
//     type Error = io::Error;
//
//     fn poll(&mut self) -> Poll<(), io::Error> {
//         loop {
//             if let Some((size, peer)) = self.to_send {
//                 let result = try_ready!(self.socket.poll_send_to(&self.buf[..size], &peer));
//                 println!("Echoed {}/{} bytes to {}", result, size, peer);
//                 self.to_send = None;
//             }
//
//             self.to_send = Some(try_ready!(self.socket.poll_recv_from(&mut self.buf)))
//         }
//     }
// }

impl Future for Server {
    type Item = ();
    type Error = io::Error;

    fn poll(&mut self) -> Poll<(), io::Error> {
        loop {
            let read_byte_count = try_ready!(self.socket.poll_recv(&mut self.buf));
            // println!("Count: {}, Bytes: {:?}", read_byte_count, &self.buf);
            if read_byte_count > 0 {
                let greeting_list = parse_from_bytes::<GreetingList>(&self.buf[0..read_byte_count]);
                println!("Got from the wire {:?}", greeting_list)
            }
            // if read_byte_count != 0 {
            //     let mut input_stream = CodedInputStream::from_bytes(&mut self.buf);
            //     greeting_list.merge_from(&mut input_stream)?;
            //     println!("Got from the wire {:?}", greeting_list)
            // }
        }
    }
}

fn main() -> Result<(), Box<error::Error>> {
    let addr = "127.0.0.1:4000".to_string().parse::<SocketAddr>()?;
    let socket = UdpSocket::bind(&addr)?;
    println!("Listening on: {}", socket.local_addr()?);

    let server = Server {
        socket: socket,
        buf: vec![0; 1024],
    };

    tokio::run(server.map_err(|e| println!("server error = {:?}", e)));
    Ok(())
}

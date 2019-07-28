extern crate protobuf;

use sidecar::hello::{Greeting, GreetingList};
use std::net::UdpSocket;

use protobuf::CodedOutputStream;
use protobuf::Message;

fn main() {
    let mut greeting_list = GreetingList::new();

    let mut greeting = Greeting::new();
    greeting.set_language("Spanish".to_string());
    greeting.set_person("Roman".to_string());

    let mut greetings = greeting_list.take_greetings();
    greetings.push(greeting);
    greeting_list.set_greetings(greetings);

    let mut bytes = Vec::new();

    let mut output_stream = CodedOutputStream::new(&mut bytes);

    match greeting_list.write_to(&mut output_stream) {
        Ok(_) => {}
        Err(err) => {
            println!("(1) nope! {:?}", err);
            return;
        }
    };

    match output_stream.flush() {
        Ok(_) => {}
        Err(err) => {
            println!("(2) nope! {:?}", err);
            return;
        }
    }

    let socket = UdpSocket::bind("127.0.0.1:3000").expect("couldn't bind to address");
    socket
        .send_to(&bytes, "127.0.0.1:4000")
        .expect("couldn't send data");
}

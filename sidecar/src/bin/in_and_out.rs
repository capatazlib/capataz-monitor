extern crate protobuf;

use sidecar::hello::{Greeting, GreetingList};

// use protobuf::stream::WithCodedInputStream;
use protobuf::{CodedOutputStream, Message};

fn main() {
    let mut greeting_list = GreetingList::new();

    let mut greeting0 = Greeting::new();
    greeting0.set_language("Spanish".to_string());
    greeting0.set_person("Roman".to_string());

    let mut greeting1 = Greeting::new();
    greeting1.set_language("English".to_string());
    greeting1.set_person("Brian".to_string());

    let mut greetings = greeting_list.take_greetings();
    greetings.push(greeting0);
    greetings.push(greeting1);
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

    println!("bytes: {:?}", bytes);

    // let result = bytes.with_coded_input_stream(|is| is.read_message::<GreetingList>());
    // match result {
    //     Ok(r) => println!("{:?}", r),
    //     Err(err) => println!("(3) Nope {:?}", err),
    // }

    let greeting_list1 = protobuf::parse_from_bytes::<GreetingList>(&bytes);
    println!("{:?}", greeting_list1);

    // let mut input_stream = CodedInputStream::from_buffered_reader(&mut bytes);
    // let greeting_list1 = input_stream.read_message::<GreetingList>();
    // println!("{:?}", greeting_list1);
}

use crate::hello::Hello;

use protobuf::error::ProtobufError;
use protobuf::{CodedInputStream, CodedOutputStream, Message, ProtobufResult};
use std::fs::File;
use std::io::{self, stdin, BufRead, BufReader};
use std::path::Path;

fn prompt_for_person() -> ProtobufResult<Hello> {
    let mut greeting = Hello::new();
    let input = stdin();

    println!("Enter new person name to greet: ");
    let mut name = String::new();
    input
        .lock()
        .read_line(&mut name)
        .map_err(ProtobufError::IoError)?;
    greeting.set_person(
        name.trim()
            .parse()
            .map_err(|e| ProtobufError::IoError(io::Error::new(io::ErrorKind::InvalidInput, e)))?,
    );

    return Ok(greeting);
}

pub fn exec(file_path: &str) -> ProtobufResult<()> {
    let mut greeting = Hello::new();

    let path = Path::new(file_path);
    if path.exists() {
        let file = File::open(&path).map_err(ProtobufError::IoError)?;
        let mut br = BufReader::new(file);
        let mut cis = CodedInputStream::from_buffered_reader(&mut br);
        greeting.merge_from(&mut cis)?;
        println!("Got from file: {:?}", greeting)
    }

    let greeting = prompt_for_person()?;
    let mut file = File::create(&path).map_err(ProtobufError::IoError)?;
    let mut cos = CodedOutputStream::new(&mut file);

    let _ = greeting.write_to(&mut cos);
    cos.flush()?;
    Ok(())
}

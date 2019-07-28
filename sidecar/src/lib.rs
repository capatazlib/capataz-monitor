pub mod hello;
// mod store;

/*
use protobuf::error::ProtobufError;
use protobuf::ProtobufResult;
use std::io::{self, stderr, Write};
use std::{env, process};

pub fn protobuf_example() {
    let args: Vec<String> = env::args().collect();
    get_exec(&args)
        .map_err(ProtobufError::IoError)
        .and_then(|(file, exec_fn)| exec_fn(file))
        .unwrap_or_else(|e| {
            stderr().write_fmt(format_args!("{}\n", e)).unwrap();
            process::exit(-1);
        })
}

pub fn get_exec(args: &Vec<String>) -> Result<(&str, fn(&str) -> ProtobufResult<()>), io::Error> {
    if args.len() != 2 {
        return Err(io::Error::new(
            io::ErrorKind::Other,
            format!("Expecting filename, none was given",),
        ));
    }
    Ok((&args[1], store::exec))
}
*/

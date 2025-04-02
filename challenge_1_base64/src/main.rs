use base64::prelude::*;
use file_handler::read_arg_file;
use hex;
use std::io::{self, prelude::*, BufReader};

fn main() {
    let reader = read_arg_file().unwrap();
    for line in reader.lines() {
        if let Ok(line) = line {
            let decoded = hex::decode(&line).expect("Decoding failed");
            let base64_line = BASE64_STANDARD.encode(&decoded);
            println!("{} {}", line, base64_line);
        }
    }
}

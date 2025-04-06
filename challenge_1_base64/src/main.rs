use base64::prelude::*;
use crypto_helper::read_arg_file;
use hex;
use std::io::{self, prelude::*, BufReader};
use std::process::exit;

fn main() {
    let mut readers = read_arg_file().unwrap();
    if readers.len() != 1 {
        exit(-1);
    }
    let reader = readers.pop().unwrap();
    for line in reader.lines() {
        if let Ok(line) = line {
            let decoded = hex::decode(&line).expect("Decoding failed");
            let base64_line = BASE64_STANDARD.encode(&decoded);
            println!("{} {}", line, base64_line);
        }
    }
}

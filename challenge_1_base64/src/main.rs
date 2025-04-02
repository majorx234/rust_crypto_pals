use file_handler::read_arg_file;
use std::io::{self, prelude::*, BufReader};

fn main() {
    let reader = read_arg_file().unwrap();
    for line in reader.lines() {
        if let Ok(line) = line {
            println!("{}", line);
        }
    }
}

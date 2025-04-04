use crypto_helper::{read_arg_file, read_char_data};
use hex;
use std::io::{self, prelude::*, BufReader};

fn main() {
    let letter_frequency_reader = read_arg_file().unwrap();
    let input_hex_string = read_char_data();
    println!("{}", String::from_iter(input_hex_string.1));
}

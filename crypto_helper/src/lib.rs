use std::env::args;
use std::fs::File;
use std::io::{self, prelude::*, BufReader};

pub fn read_arg_file() -> io::Result<BufReader<File>> {
    let mut argit = args();
    let file_name = argit.nth(1).clone();

    let file_name = if let Some(file_name) = file_name {
        file_name
    } else {
        panic!("No filename argument given");
    };

    let file = File::open(file_name)?;
    Ok(BufReader::new(file))
}

pub fn read_char_data() -> (usize, std::vec::Vec<char>) {
    let mut vec_values = Vec::new();
    let result_size = std::io::stdin().lock().read_to_end(&mut vec_values);
    let num_samples = result_size.expect("no input");
    (num_samples, vec_values.iter().map(|x| *x as char).collect())
}

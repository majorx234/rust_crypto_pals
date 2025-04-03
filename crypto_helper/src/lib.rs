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

pub fn read_data() -> (usize, std::vec::Vec<f32>) {
    let values_data = std::io::stdin()
        .lock()
        .lines()
        .map(|x| x.expect("0.0").parse::<f32>().unwrap())
        .collect::<Vec<f32>>();
    let num_samples = values_data.len();
    return (num_samples, values_data);
}

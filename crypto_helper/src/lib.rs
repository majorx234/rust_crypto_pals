use std::collections::HashMap;
use std::env::args;
use std::fs::File;
use std::io::{self, prelude::*, BufReader};

pub fn read_arg_file() -> io::Result<Vec<BufReader<File>>> {
    let mut argit = args();
    let mut buf_vec = Vec::new();
    let file_name = argit.nth(1).clone();

    let file_name = if let Some(file_name) = file_name {
        file_name
    } else {
        panic!("No filename argument given");
    };

    let file = File::open(file_name)?;
    buf_vec.push(BufReader::new(file));
    if args().len() > 2 {
        while let Some(file_name) = argit.next().clone() {
            let file = File::open(file_name)?;
            buf_vec.push(BufReader::new(file));
        }
    }
    Ok(buf_vec)
}

pub fn read_char_data() -> (usize, std::vec::Vec<char>) {
    let mut vec_values = Vec::new();
    let result_size = std::io::stdin().lock().read_to_end(&mut vec_values);
    let num_samples = result_size.expect("no input");
    (num_samples, vec_values.iter().map(|x| *x as char).collect())
}

pub fn read_letter_frequency_map_from_file(
    file: BufReader<File>,
    delimitter: char,
) -> Option<HashMap<char, f32>> {
    let mut hash_map = HashMap::new();
    for line in file.lines() {
        match line {
            Ok(line) => {
                if let Some((key_str, value_str)) = line.split_once(delimitter) {
                    if let (Ok(key), Ok(value)) =
                        (key_str.parse::<char>(), value_str.parse::<f32>())
                    {
                        hash_map.insert(key, value);
                    } else {
                        return None;
                    }
                }
            }
            _ => (),
        }
    }
    Some(hash_map)
}
pub fn read_letter_replace_map_from_file(
    file: BufReader<File>,
    delimitter: char,
) -> Option<HashMap<u8, char>> {
    let mut hash_map = HashMap::new();
    for line in file.lines() {
        match line {
            Ok(line) => {
                if let Some((key_str, value_str)) = line.split_once(delimitter) {
                    if let (Ok(key), Ok(value)) = (key_str.parse::<u8>(), value_str.parse::<char>())
                    {
                        hash_map.insert(key, value);
                    } else {
                        return None;
                    }
                }
            }
            _ => (),
        }
    }
    Some(hash_map)
}

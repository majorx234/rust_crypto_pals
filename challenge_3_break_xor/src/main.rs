use crypto_helper::{
    read_arg_file, read_char_data, read_letter_frequency_map_from_file,
    read_letter_replace_map_from_file,
};
use hex;
use std::collections::hash_map::HashMap;
use std::io::{self, prelude::*, BufReader};

fn main() {
    let mut files = read_arg_file().unwrap();
    let letter_replace_map_reader = files.pop().expect("no letter replace map");
    let letter_frequency_reader = files.pop().expect("no letter frequency map");
    let (input_size, mut input_hex_vec_raw) = read_char_data();
    println!(
        "raw: {} {}",
        String::from_iter(&input_hex_vec_raw),
        input_size
    );
    input_hex_vec_raw.retain(|&x| x != '\n');

    let input_hex_vec = hex::decode(String::from_iter(input_hex_vec_raw)).expect("Hex error");
    let mut input_frequency_map: HashMap<u8, u16> = HashMap::new();
    for letter in input_hex_vec {
        let value = input_frequency_map.entry(letter).or_insert_with(|| 0);
        *value += 1;
    }
    let mut letter_frequency_list = Vec::new();
    for entry in input_frequency_map {
        letter_frequency_list.push(entry);
    }
    letter_frequency_list.sort_by_key(|k| k.1);
    println!("letter frequency: {:?}", letter_frequency_list);

    let letter_frequency_map = read_letter_frequency_map_from_file(letter_frequency_reader, ',');
    let letter_replace_map = read_letter_replace_map_from_file(letter_replace_map_reader, ',');
    println!("letter frequency_map: {:?}", letter_frequency_map);
    println!("letter repalace: {:?}", letter_replace_map);
}

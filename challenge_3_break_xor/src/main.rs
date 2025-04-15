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
    for letter in &input_hex_vec {
        let value = input_frequency_map.entry(*letter).or_insert_with(|| 0);
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

    if let Some(letter_replace_map) = letter_replace_map {
        print_decrpyted_input(input_hex_vec, &letter_replace_map);
    }
}

fn print_decrpyted_input(input_hex_vec: Vec<u8>, letter_replace_map: &HashMap<u8, char>) {
    for letter_code in input_hex_vec {
        if let Some(replace_letter) = letter_replace_map.get(&letter_code) {
            print!("{}", replace_letter);
        } else {
            print!("[{}]", letter_code);
        }
    }
    println!("\n");
}

fn brute_force_score_string(input_hex_vec: &[u8], letter_frequency_map: &HashMap<char, f32>) -> u8 {
    let mut max = 0.0;
    let mut best_xor_key = 0;
    for xor_key in 0..=255 {
        let mut num_letters = 0;
        let word = input_hex_vec
            .iter()
            .map(|x| *x ^ xor_key)
            .collect::<Vec<u8>>();
        let mut curent_frequency_map: HashMap<char, f32> = HashMap::new();
        for letter in word {
            if letter.is_ascii() {
                let letter_char = letter.to_ascii_lowercase() as char;
                let value = curent_frequency_map
                    .entry(letter_char)
                    .or_insert_with(|| 0.0);
                *value += 1.0;
                num_letters += 1;
            }
        }
        if num_letters != 0 {
            // normalize
            for (key, value) in curent_frequency_map.iter_mut() {
                *value /= num_letters as f32;
            }
        } else {
            break;
        }
        let distance = bhattacharyya_distance(&letter_frequency_map, &curent_frequency_map);
        if max < 1.0 / distance {
            max = 1.0 / distance;
            best_xor_key = xor_key;
        }
    }
    best_xor_key
}

fn decrypt_text(input_hex_vec: Vec<u8>, xor_key: u8) -> Vec<char> {
    input_hex_vec
        .iter()
        .map(|x| (*x ^ xor_key).to_ascii_lowercase() as char)
        .collect::<Vec<char>>()
}

fn bhattacharyya_distance(dist1: &HashMap<char, f32>, dist2: &HashMap<char, f32>) -> f32 {
    let mut bc_coeff = 0.0;
    for c in b'a'..=b'z' {
        let fact1 = dist1[&(c as char)];
        let fact2 = dist2[&(c as char)];
        bc_coeff += (fact1 * fact2).sqrt();
    }
    bc_coeff
}

use hex;
use std::env::args;
use std::process::exit;
fn main() {
    let mut argit = args();
    let args_length = args().len();
    if args_length != 3 {
        println!("need two args");
        exit(1);
    }
    let first = argit.nth(1).clone();
    let second = argit.next().clone();
    if let (Some(first), Some(second)) = (first, second) {
        if first.len() != second.len() {
            println!("input need to have same length");
            exit(2);
        }
        let first_decoded = hex::decode(&first).expect("Decoding failed");
        let second_decoded = hex::decode(&second).expect("Decoding failed");
        let mut xor_vec: Vec<u8> = Vec::new();
        for (f, s) in first_decoded.iter().zip(second_decoded) {
            xor_vec.push(f ^ s);
        }
        let xor_string = hex::encode(xor_vec);
        println!("{}", xor_string);
    } else {
        println!("something went terribly wroing");
    }
}

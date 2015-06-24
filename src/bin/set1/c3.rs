extern crate crypto;
extern crate rustc_serialize;

use std::io;
    
use crypto::crypto::freq_scoring::{get_best_fit};
use crypto::util::{hex_string_to_bytes};

fn main() {
    let mut hex = String::new();

    io::stdin().read_line(&mut hex)
        .ok()
        .expect("Failed to read hex input");

    let result = get_best_fit(&hex_string_to_bytes(&hex));
    println!("Decoded: {}\nScore: {}", result.decoded, result.score);
}


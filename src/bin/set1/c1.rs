extern crate cryptopals;

use std::io;
use cryptopals::util::{hex_string_to_base64};

fn main() {
    let mut hex = String::new();

    io::stdin().read_line(&mut hex)
        .ok()
        .expect("Failed to read hex input");

    println!("Hex: {}", hex);

    let encoded = hex_string_to_base64(&hex);

    println!("Encoded: {}", encoded);
}

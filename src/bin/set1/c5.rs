extern crate crypto;

use std::io;
use std::io::Read;

use crypto::util::{string_repeating_xor};

fn main() {
    let mut message = String::new();
    io::stdin().read_to_string(&mut message)
        .ok()
        .expect("Failed to read message content.");
    let encoded = string_repeating_xor(&message, &"ICE".to_string());
    println!("{}", encoded);
}


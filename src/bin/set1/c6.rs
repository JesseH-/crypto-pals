extern crate crypto;

use std::io;
use std::io::Read;

use crypto::crypto::crack::{break_repeating_key_xor};
use crypto::util::{base64_string_to_bytes};

fn main() {
    let mut message = String::new();
    io::stdin().read_to_string(&mut message)
        .ok()
        .expect("Failed to read message content.");
    let hex = base64_string_to_bytes(&message);
    let cracked = break_repeating_key_xor(&hex);
    println!("{}", cracked.decoded);
}


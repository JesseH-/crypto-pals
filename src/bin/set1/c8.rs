extern crate crypto;
extern crate cryptopals;

use std::io;
use std::io::BufRead;

use cryptopals::util::{has_repeated_blocks, hex_string_to_bytes};

fn main() {
    let stdin = io::stdin();
    for line in stdin.lock().lines() {
        let encrypted = line.unwrap();
        let hex_bytes = hex_string_to_bytes(&encrypted);
        if has_repeated_blocks(&hex_bytes, 16) {
            println!("{}", encrypted);
        }
    }
}


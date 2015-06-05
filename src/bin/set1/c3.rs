extern crate crypto;
extern crate rustc_serialize;

use std::io;
use std::cmp::Ordering::{Equal};
    
use crypto::crypto::freq_scoring::{score_freq};
use crypto::util::{hex_string_xor};
use rustc_serialize::hex::{FromHex, ToHex};

fn main() {
    let mut hex = String::new();

    io::stdin().read_line(&mut hex)
        .ok()
        .expect("Failed to read hex input");

    let mut xors = Vec::new();
    for i in 0..128 {
        let u = i as u8;
        let rep = (0..hex.len()).map(|_| u).collect::<Vec<u8>>().to_hex();
        let bytes = hex_string_xor(&hex, &rep).from_hex().unwrap();
        let result = String::from_utf8(bytes).unwrap();
        xors.push((i, result));
    }

    xors.sort_by(|&(i, ref x), &(j, ref y)|
                 (&score_freq(&y)).partial_cmp(&score_freq(&x))
                 .unwrap_or(Equal));
    println!("Decoded: {}\nScore: {}", xors[0].1, score_freq(&xors[0].1));
}


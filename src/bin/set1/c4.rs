extern crate crypto;
extern crate rustc_serialize;

use std::cmp::Ordering::{Equal,Greater};
use std::io;
use std::io::{BufRead};

use crypto::crypto::freq_scoring::{get_best_fit, Fit};
use crypto::util::{hex_string_to_bytes};

fn main() {
    let mut best = Fit { .. Default::default() };
    let stdin = io::stdin();
    for line in stdin.lock().lines() {
        let result = get_best_fit(&hex_string_to_bytes(&line.unwrap()));
        if result.score.partial_cmp(&best.score).unwrap_or(Equal) == Greater {
            best = Fit { .. result };
        };
    }

    println!("Decoded: {}\nScore: {}", best.decoded, best.score);
}

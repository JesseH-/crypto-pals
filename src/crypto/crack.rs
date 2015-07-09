use std::cmp::{min};
use std::cmp::Ordering::{Equal, Greater};

use crypto::encrypt::{append_ecb_encrypt};
use crypto::freq_scoring::{score_freq, get_best_fit, Fit};
use util::{edit_distance, repeating_xor};

pub struct KeyFit {
    pub size: usize,
    pub score: f64
}

fn key_fits(message: &[u8]) -> Vec<KeyFit> {
    let mut keys = Vec::new();
    for i in 2 .. 40 {
        let size = i as usize;
        let norm = i as f64;
        let mut score = 0.0;
        for j in 0 .. 4 {
            let slice1 = &message[size * j .. size * (j + 1)];
            let slice2 = &message[size * (j + 1) .. size * (j + 2)];
            score += edit_distance(slice1, slice2) as f64;
        }
        score = score / 4.0 / norm;
        keys.push(KeyFit { size: size, score: score });
    }
    keys.sort_by(|x, y| x.score.partial_cmp(&y.score).unwrap_or(Equal));
    keys
}

fn make_blocks(message: &[u8], key_size: usize) -> Vec<Vec<u8>> {
    let mut blocks = Vec::new();
    for _ in 0 .. key_size {
        blocks.push(Vec::new());
    }
    for i in 0 .. message.len()/key_size {
        let start = i * key_size;
        let slice = &message[start .. start + key_size];
        let mut count = 0;
        for c in slice.iter() {
            blocks[count].push(*c);
            count = (count + 1) % key_size;
        }
    }
    blocks
}

pub fn break_repeating_key_xor(message: &[u8]) -> Fit {
    let mut best = Fit { .. Default::default() };
    let fits = key_fits(message);
    for i in 0 .. min(5, fits.len()) {
        let ref key_fit = fits[i];
        let blocks = make_blocks(message, key_fit.size);
        let mut key = Vec::new();
        for pad in blocks.iter().map(|s| get_best_fit(s).pad) {
            for byte in pad {
                key.push(byte);
            }
        }
        let decoded = String::from_utf8(repeating_xor(message, &key))
            .ok()
            .expect("Could not decode string");
        let fit = score_freq(&decoded);
        if fit.partial_cmp(&best.score).unwrap_or(Equal) == Greater {
            best = Fit { score: fit, decoded: decoded, pad: key };
        }
    }
    best
}

fn find_key_size(append: &[u8], key: &[u8]) -> usize {
    let mut size = 0;
    let zeroes = vec![0u8; 64];
    let start = append_ecb_encrypt(&[0], append, key).len();
    for i in 1 .. zeroes.len() {
        let end = append_ecb_encrypt(&zeroes[0 .. i], append, key).len();
        if start != end {
            size = end - start;
            break;
        }
    }
    size
}

#[test]
fn test_find_key_size() {
    let key = [9u8; 16];
    for i in 1 .. 64 {
        let append = vec![7u8; i];
        assert_eq!(16, find_key_size(&append, &key));
    }
}

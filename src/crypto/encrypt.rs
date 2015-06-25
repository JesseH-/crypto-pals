extern crate rand;

use self::rand::{thread_rng, Rng};

pub fn generate_key() -> Vec<u8> {
    let mut v = vec![0u8; 16];
    thread_rng().fill_bytes(&mut v);
    v
}

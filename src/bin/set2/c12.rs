extern crate cryptopals;

use cryptopals::crypto::crack::{break_ecb};
use cryptopals::crypto::encrypt::{generate_key};
use cryptopals::util::{base64_string_to_bytes};

fn main() {
    let encode = "Um9sbGluJyBpbiBteSA1LjAKV2l0aCBteSByYWctdG9wIGRvd24gc28gbX\
                  kgaGFpciBjYW4gYmxvdwpUaGUgZ2lybGllcyBvbiBzdGFuZGJ5IHdhdmluZ\
                  yBqdXN0IHRvIHNheSBoaQpEaWQgeW91IHN0b3A/IE5vLCBJIGp1c3QgZHJv\
                  dmUgYnkK";
    let bytes = base64_string_to_bytes(&encode);
    let key = generate_key();
    let secret_bytes = break_ecb(&bytes, &key);
    let secret = String::from_utf8(secret_bytes).unwrap();
    println!("{:?}", secret);
}

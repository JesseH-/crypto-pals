extern crate crypto;
extern crate cryptopals;

use std::io;
use std::io::Read;

use cryptopals::crypto::decrypt::{decrypt_aes_ecb};
use cryptopals::util::{base64_string_to_bytes};

fn main() {
    let mut message = String::new();
    io::stdin().read_to_string(&mut message)
        .ok()
        .expect("Failed to read message content.");
    let hex = base64_string_to_bytes(&message);
    let key = "YELLOW SUBMARINE".as_bytes();
    let decrypted_bytes = decrypt_aes_ecb(&hex, &key).ok().unwrap();
    let decrypted = String::from_utf8(decrypted_bytes).ok().unwrap();
    println!("{}", decrypted);
}


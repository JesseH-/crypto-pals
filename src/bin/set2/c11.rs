extern crate cryptopals;

use std::io;
use std::io::Read;

use cryptopals::crypto::encrypt::{random_encrypt, Mode};
use cryptopals::util::{has_repeated_blocks};

fn main() {
    let mut message = String::new();
    io::stdin().read_to_string(&mut message)
        .ok()
        .expect("Failed to read message content.");
    let plaintext = message.into_bytes();
    for _ in 0 .. 65536 {
        let result = random_encrypt(&plaintext);
        let mode;
        if has_repeated_blocks(&result.encrypted, 16) {
            mode = Mode::ECB;
        } else {
            mode = Mode::CBC;
        }
        assert!(mode == result.mode);
    }
}

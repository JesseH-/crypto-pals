extern crate cryptopals;

use cryptopals::crypto::encrypt::{random_encrypt, Mode};
use cryptopals::util::{has_repeated_blocks};

#[test]
fn test_encryption_oracle() {
    let plaintext = vec![0u8; 64];
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

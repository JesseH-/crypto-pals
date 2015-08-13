extern crate cryptopals;

use cryptopals::crypto::crack::{cbc_bitflip_attack,
                                ecb_cut_and_paste_break_profile};
use cryptopals::crypto::encrypt::{random_encrypt, Mode};
use cryptopals::util::{has_repeated_blocks};
use cryptopals::util::cookie::{encode_profile};

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

#[test]
fn test_ecb_cut_and_paste() {
    let key = "YELLOW SUBMARINE".to_string().into_bytes();
    let profile = ecb_cut_and_paste_break_profile(&key);
    assert_eq!(profile.get("role").unwrap(), "admin");
    assert_eq!(profile.get("email").unwrap(), "foo@domain.ca");
    assert_eq!(encode_profile(&profile),
               "email=foo@domain.ca&uid=10&role=admin");
}

#[test]
fn test_cbc_bitflip_attack() {
    let key = "YELLOW SUBMARINE".to_string().into_bytes();
    let iv = "foobarfoobarfoob".to_string().into_bytes();
    assert!(cbc_bitflip_attack(&key, &iv));
}

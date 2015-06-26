extern crate cryptopals;

use cryptopals::crypto::decrypt::{decrypt_aes_ecb, pkcs_pad};
use cryptopals::crypto::encrypt::{encrypt_aes_ecb};

#[test]
fn test_ecb_encrypt() {
    let decoded = "YELLOW SUBMARINE".to_string();
    let mut key = "AA".to_string().into_bytes();
    pkcs_pad(&mut key, 16);
    let encrypted = encrypt_aes_ecb(&decoded.as_bytes(), &key).ok().unwrap();
    let decrypted = decrypt_aes_ecb(&encrypted, &key).ok().unwrap();
    let result = String::from_utf8(decrypted).ok().unwrap();
    assert_eq!(decoded, result);
}

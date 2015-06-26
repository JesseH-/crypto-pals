extern crate cryptopals;

use cryptopals::crypto::decrypt::{decrypt_aes_ecb, pkcs_unpad};
use cryptopals::crypto::encrypt::{encrypt_aes_ecb, pkcs_pad};

#[test]
fn test_pkcs_pad() {
    let expected = "YELLOW SUBMARINE\x04\x04\x04\x04".to_string().into_bytes();
    let mut bytes = "YELLOW SUBMARINE".to_string().into_bytes();
    pkcs_pad(&mut bytes, 20);
    assert_eq!(bytes, expected);
}

#[test]
fn test_pkcs_unpad() {
    let expected = "YELLOW SUBMARINE".to_string().into_bytes();
    let mut bytes = "YELLOW SUBMARINE\x04\x04\x04\x04".to_string().into_bytes();
    pkcs_unpad(&mut bytes);
    assert_eq!(bytes, expected);
}

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

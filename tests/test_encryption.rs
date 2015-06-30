extern crate cryptopals;

use cryptopals::crypto::decrypt::{decrypt_aes_ecb, decrypt_aes_cbc, pkcs_unpad};
use cryptopals::crypto::encrypt::{encrypt_aes_ecb, encrypt_aes_cbc, pkcs_pad};

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
fn test_ecb_encryption() {
    let decoded = "YELLOW SUBMARINE".to_string();
    let mut key = "AA".to_string().into_bytes();
    pkcs_pad(&mut key, 16);
    let encrypted = encrypt_aes_ecb(&decoded.as_bytes(), &key).ok().unwrap();
    let decrypted = decrypt_aes_ecb(&encrypted, &key).ok().unwrap();
    let result = String::from_utf8(decrypted).ok().unwrap();
    assert_eq!(decoded, result);
}

#[test]
fn test_ecb_encryption_unaligned() {
    let decoded = "YELLOW SUBMARINEEEEE".to_string();
    let mut plaintext = decoded.clone().into_bytes();
    let mut key = "AA".to_string().into_bytes();
    pkcs_pad(&mut plaintext, 16);
    pkcs_pad(&mut key, 16);
    let encrypted = encrypt_aes_ecb(&plaintext, &key).ok().unwrap();
    let mut decrypted = decrypt_aes_ecb(&encrypted, &key).ok().unwrap();
    pkcs_unpad(&mut decrypted);
    let result = String::from_utf8(decrypted).ok().unwrap();
    assert_eq!(decoded, result);
}

#[test]
fn test_cbc_encryption() {
    let decoded = "YELLOW SUBMARINELOW SUBMARINELOW SUBMARINE".to_string();
    let key = "ABCDEFGHIJKLMNOP".to_string().into_bytes();
    let iv = "ZYXWVUTSRQPONMLK".to_string().into_bytes();
    let mut plaintext = decoded.clone().into_bytes();
    pkcs_pad(&mut plaintext, 16);
    let encrypted = encrypt_aes_cbc(&plaintext, &key, &iv);
    let mut decrypted = decrypt_aes_cbc(&encrypted, &key, &iv);
    pkcs_unpad(&mut decrypted);
    let result = String::from_utf8(decrypted).ok().unwrap();
    assert_eq!(decoded, result);
}

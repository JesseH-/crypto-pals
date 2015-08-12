extern crate cryptopals;

use cryptopals::crypto::decrypt::{decrypt_aes_ecb, decrypt_aes_cbc,
                                  decrypt_profile, pkcs_unpad};
use cryptopals::crypto::encrypt::{encrypt_aes_ecb, encrypt_aes_cbc,
                                  generate_encrypted_profile, pkcs_pad};
use cryptopals::util::cookie::{encode_profile};

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
    pkcs_unpad(&mut bytes).unwrap();
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
    pkcs_unpad(&mut decrypted).unwrap();
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
    pkcs_unpad(&mut decrypted).unwrap();
    let result = String::from_utf8(decrypted).ok().unwrap();
    assert_eq!(decoded, result);
}

#[test]
fn test_pkcs_valid_padding() {
    let mut decoded = "ICE ICE BABY\x04\x04\x04\x04".to_string().into_bytes();
    let result = pkcs_unpad(&mut decoded);
    assert!(result.is_ok());
}

#[test]
fn test_pkcs_short_padding() {
    let mut decoded = "ICE ICE BABY\x05\x05\x05\x05".to_string().into_bytes();
    let result = pkcs_unpad(&mut decoded);
    assert!(result.is_err());
}

#[test]
fn test_pkcs_wrong_padding() {
    let mut decoded = "ICE ICE BABY\x01\x02\x03\x04".to_string().into_bytes();
    let result = pkcs_unpad(&mut decoded);
    assert!(result.is_err());
}

#[test]
fn test_profile_encryption() {
    let email = "foo@bar.com";
    let key = "YELLOW SUBMARINE".to_string().into_bytes();
    let encrypted = generate_encrypted_profile(&email, &key);
    let decrypted = decrypt_profile(&encrypted, &key);
    let profile = encode_profile(&decrypted);
    assert_eq!(profile, "email=foo@bar.com&uid=10&role=user");
}

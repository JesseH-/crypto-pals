extern crate cryptopals;

use cryptopals::crypto::decrypt::{pkcs_pad};

#[test]
fn test_pkcs_pad() {
    let expected = "YELLOW SUBMARINE\x04\x04\x04\x04".to_string().into_bytes();
    let mut bytes = "YELLOW SUBMARINE".to_string().into_bytes();
    pkcs_pad(&mut bytes, 20);
    assert_eq!(bytes, expected);
}

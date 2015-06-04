extern crate crypto;

use crypto::util::{hex_string_to_base64};

#[test]
fn test_hex_to_base64() {
    let hex = "49276d206b696c6c696e6720796f757220627261696e206c".to_string() +
        "696b65206120706f69736f6e6f7573206d757368726f6f6d";
    let encoded = hex_string_to_base64(&hex);
    assert_eq!(encoded, ("SSdtIGtpbGxpbmcgeW91ciBicmFpbiBsaWtlIG".to_string() +
                         "EgcG9pc29ub3VzIG11c2hyb29t"));
}

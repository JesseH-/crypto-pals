extern crate cryptopals;

use std::collections::HashMap;

use cryptopals::util::cookie::{encode_cookie, parse_cookie, profile_for};
use cryptopals::util::{hex_string_to_base64, hex_string_xor, string_edit_distance};

#[test]
fn test_hex_to_base64() {
    let hex = "49276d206b696c6c696e6720796f757220627261696e206c".to_string() +
        "696b65206120706f69736f6e6f7573206d757368726f6f6d";
    let encoded = hex_string_to_base64(&hex);
    assert_eq!(encoded, ("SSdtIGtpbGxpbmcgeW91ciBicmFpbiBsaWtlIG".to_string() +
                         "EgcG9pc29ub3VzIG11c2hyb29t"));
}

#[test]
fn test_hex_xor() {
    let hex1 = "1c0111001f010100061a024b53535009181c";
    let hex2 = "686974207468652062756c6c277320657965";
    let result = hex_string_xor(&hex1.to_string(), &hex2.to_string());
    assert_eq!(result, "746865206b696420646f6e277420706c6179");
}

#[test]
fn test_edit_distance() {
    let string1 = "this is a test".to_string();
    let string2 = "wokka wokka!!!".to_string();
    assert_eq!(string_edit_distance(&string1, &string2), 37);
}

#[test]
fn test_parse_cookie() {
    let string = "foo=bar&baz=qux&zap=zazzle";
    let mut map = HashMap::new();
    map.insert("foo".to_string(), "bar".to_string());
    map.insert("baz".to_string(), "qux".to_string());
    map.insert("zap".to_string(), "zazzle".to_string());
    assert_eq!(map, parse_cookie(&string));
}

#[test]
fn test_encode_cookie() {
    let mut map = HashMap::new();
    map.insert("foo".to_string(), "bar".to_string());
    map.insert("baz".to_string(), "qux".to_string());
    map.insert("zap".to_string(), "zazzle".to_string());
    assert_eq!(map, parse_cookie(&encode_cookie(&map)));
}

#[test]
fn test_profile_for() {
    let email = "foo@bar.com";
    assert_eq!(profile_for(&email).unwrap(),
               "email=foo@bar.com&uid=10&role=user");
}

#[test]
fn test_profile_for_illegal_characters() {
    let email1 = "foo&";
    let email2 = "bar=";
    assert!(profile_for(&email1).is_err());
    assert!(profile_for(&email2).is_err());
}

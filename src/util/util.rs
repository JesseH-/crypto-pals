use rustc_serialize::base64::{ToBase64, STANDARD};
use rustc_serialize::hex::{FromHex, ToHex};

pub fn hex_string_to_bytes(hex: &str) -> Vec<u8> {
    hex.from_hex()
        .ok()
        .expect("Failed to convert string from hex")
}

pub fn fixed_xor(v1: &[u8], v2: &[u8]) -> Vec<u8> {
    v1.iter().zip(v2.iter()).map(|(x, y)| *x ^ *y).collect::<Vec<u8>>()
}

pub fn repeating_xor(bytes: &[u8], key: &[u8]) -> Vec<u8> {
    bytes.iter().zip(key.iter().cycle()).map(|(x, y)| *x ^ *y)
        .collect::<Vec<u8>>()
}

fn bit_count(x: u64) -> u64 {
    let y = x - ((x >> 1) & 0x55555555);
    let y = (y & 0x33333333) + ((y >> 2) & 0x33333333);
    (((y + (y >> 4)) & 0x0f0f0f0f) * 0x01010101) >> 24
}

pub fn edit_distance(bytes1: &[u8], bytes2: &[u8]) -> u64 {
    let xored = bytes1.iter().zip(bytes2.iter()).map(|(&x, &y)| x ^ y)
        .collect::<Vec<u8>>();
    let mut i = 0;
    for u in xored.iter().map(|&x| bit_count(x as u64)) {
        i += u
    }
    i
}

pub fn hex_string_to_base64(hex: &str) -> String {
    let bytes = hex_string_to_bytes(hex);
    bytes.to_base64(STANDARD)
}

pub fn hex_string_xor(hex1: &str, hex2: &str) -> String {
    let bytes1 = hex_string_to_bytes(hex1);
    let bytes2 = hex_string_to_bytes(hex2);
    let xor = fixed_xor(&bytes1, &bytes2);
    xor.to_hex()
}

pub fn string_repeating_xor(message: &str, key: &str) -> String {
    repeating_xor(message.as_bytes(), key.as_bytes()).to_hex()
}

pub fn string_edit_distance(string1: &str, string2: &str) -> u64 {
    let bytes1 = string1.as_bytes();
    let bytes2 = string2.as_bytes();
    edit_distance(bytes1, bytes2)
}

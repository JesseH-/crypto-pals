use rustc_serialize::base64::{ToBase64, STANDARD};
use rustc_serialize::hex::{FromHex, ToHex};

fn hex_string_to_bytes(hex: &String) -> Vec<u8> {
    hex.from_hex()
        .ok()
        .expect("Failed to convert string to hex")
}

fn fixed_xor(v1: &Vec<u8>, v2: &Vec<u8>) -> Vec<u8> {
    v1.iter().zip(v2.iter()).map(|(x, y)| *x ^ *y).collect::<Vec<u8>>()
}

pub fn hex_string_to_base64(hex: &String) -> String {
    let bytes = hex_string_to_bytes(hex);
    bytes.to_base64(STANDARD)
}

pub fn hex_string_xor(hex1: &String, hex2: &String) -> String {
    let bytes1 = hex_string_to_bytes(hex1);
    let bytes2 = hex_string_to_bytes(hex2);
    let xor = fixed_xor(&bytes1, &bytes2);
    xor.to_hex()
}

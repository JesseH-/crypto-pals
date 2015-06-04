use rustc_serialize::base64::{ToBase64, STANDARD};
use rustc_serialize::hex::FromHex;

pub fn hex_string_to_base64(hex: &String) -> String {
    let encoded = hex.from_hex()
        .ok()
        .expect("Failed to convert string to hex");
    encoded.to_base64(STANDARD)
}

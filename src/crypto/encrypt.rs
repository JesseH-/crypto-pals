extern crate crypto;
extern crate rand;

use std::cmp::{min};

use self::crypto::{symmetriccipher, buffer, aes, blockmodes};
use self::crypto::buffer::{ReadBuffer, WriteBuffer, BufferResult};
use self::rand::{thread_rng, Rng};

use util::{concat_bytes, fixed_xor};
use util::cookie::{profile_for};

#[derive(PartialEq)]
pub enum Mode {
    ECB,
    CBC,
}

pub struct RandomResult {
    pub encrypted: Vec<u8>,
    pub mode: Mode,
}

pub fn generate_key() -> Vec<u8> {
    let mut v = vec![0u8; 16];
    thread_rng().fill_bytes(&mut v);
    v
}

pub fn encrypt_aes_ecb(plaintext: &[u8], key: &[u8])
                       -> Result<Vec<u8>, symmetriccipher::SymmetricCipherError>
{
    assert!(plaintext.len() % key.len() == 0);
    let mut encryptor = aes::ecb_encryptor(aes::KeySize::KeySize128,
                                           key, blockmodes::NoPadding);

    let mut final_result = Vec::<u8>::new();
    let mut read_buf = buffer::RefReadBuffer::new(plaintext);
    let mut buf = [0; 4096];
    let mut write_buf = buffer::RefWriteBuffer::new(&mut buf);

    loop {
        let result = try!(encryptor.encrypt(&mut read_buf, &mut write_buf,
                                            true));
        final_result.extend(write_buf
                            .take_read_buffer()
                            .take_remaining()
                            .iter().map(|&i| i));
        match result {
            BufferResult::BufferUnderflow => break,
            BufferResult::BufferOverflow => { }
        }
    }

    Ok(final_result)
}

pub fn encrypt_aes_cbc(plaintext: &[u8], key: &[u8], iv: &[u8]) -> Vec<u8>{
    assert!(key.len() == iv.len());
    assert!(plaintext.len() % key.len() == 0);
    let block_size = key.len();
    let mut final_result = Vec::<u8>::new();

    let mut carry = iv.to_vec();
    for i in 0 .. (plaintext.len() + block_size - 1) / block_size {
        let end = min((i + 1) * block_size, plaintext.len());
        let next_block = &plaintext[i * block_size .. end];
        let block = fixed_xor(next_block, &carry);

        carry = encrypt_aes_ecb(&block, key).unwrap();
        final_result.extend(carry.iter().cloned());
    }
    final_result
}

pub fn pkcs_pad(blocks: &mut Vec<u8>, length: usize) {
    let difference = length - (blocks.len() % length);
    for _ in 0 .. difference {
        blocks.push(difference as u8);
    }
}

fn random_pad(plaintext: &[u8]) -> Vec<u8> {
    let mut rng = thread_rng();
    let mut decoded = Vec::new();

    for _ in 0 .. rng.gen_range(5, 11) {
        decoded.push(rng.gen::<u8>());
    }

    concat_bytes(&mut decoded, &plaintext);

    for _ in 0 .. rng.gen_range(5, 11) {
        decoded.push(rng.gen::<u8>());
    }
    decoded
}

pub fn random_encrypt(plaintext: &[u8]) -> RandomResult {
    let key = generate_key();
    let mut padded = random_pad(plaintext);
    pkcs_pad(&mut padded, key.len());

    let mut result;
    if thread_rng().gen::<bool>() {
        let iv = generate_key();
        result = RandomResult { encrypted: encrypt_aes_cbc(&padded, &key, &iv),
                                mode: Mode::CBC };
    } else {
        let encrypted = encrypt_aes_ecb(&padded, &key).unwrap();
        result = RandomResult { encrypted: encrypted,
                                mode: Mode::ECB };
    }
    result
}

pub fn append_ecb_encrypt(plaintext: &[u8], append: &[u8], key: &[u8])
                          -> Vec<u8>
{
    let mut decoded = plaintext.to_vec();
    concat_bytes(&mut decoded, &append);
    pkcs_pad(&mut decoded, key.len());
    encrypt_aes_ecb(&decoded, key).unwrap()
}

pub fn generate_encrypted_profile(email: &str, key: &[u8]) -> Vec<u8> {
    let profile = profile_for(&email).unwrap();
    let mut bytes = profile.into_bytes();
    pkcs_pad(&mut bytes, key.len());
    encrypt_aes_ecb(&bytes, &key).unwrap()
}

fn escape_characters(input: &mut String) {
    input.replace(";", "%3B");
    input.replace("=", "%3D");
}

pub fn bitflip_encrypt(plaintext: &str, key: &[u8], iv: &[u8]) -> Vec<u8> {
    let mut escaped = String::from(plaintext);
    escape_characters(&mut escaped);
    let mut bytes = "comment1=cooking%20MCs;userdata="
        .to_string()
        .into_bytes();
    let append_bytes = ";comment2=%20like%20a%20pound%20of%20bacon"
        .to_string()
        .into_bytes();
    concat_bytes(&mut bytes, &escaped.into_bytes());
    concat_bytes(&mut bytes, &append_bytes);
    pkcs_pad(&mut bytes, key.len());
    encrypt_aes_cbc(&bytes, &key, &iv)
}

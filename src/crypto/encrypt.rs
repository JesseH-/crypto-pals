extern crate crypto;
extern crate rand;

use std::cmp::{min};

use self::crypto::{symmetriccipher, buffer, aes, blockmodes};
use self::crypto::buffer::{ReadBuffer, WriteBuffer, BufferResult};
use self::rand::{thread_rng, Rng};

use util::{fixed_xor};

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

pub fn encrypt_aes_ecb(plaintext: &[u8], key: &[u8]) ->
    Result<Vec<u8>, symmetriccipher::SymmetricCipherError> {
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
    for i in 0 .. (plaintext.len() + block_size -1) / block_size {
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

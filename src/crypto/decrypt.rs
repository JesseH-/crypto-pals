extern crate crypto;

use self::crypto::{symmetriccipher, buffer, aes, blockmodes};
use self::crypto::buffer::{ReadBuffer, WriteBuffer, BufferResult};

use util::{fixed_xor};

pub fn decrypt_aes_ecb(encrypted: &[u8], key: &[u8]) ->
    Result<Vec<u8>, symmetriccipher::SymmetricCipherError> {
    assert!(encrypted.len() % key.len() == 0);
    let mut decryptor = aes::ecb_decryptor(aes::KeySize::KeySize128,
                                           key, blockmodes::NoPadding);

    let mut final_result = Vec::<u8>::new();
    let mut read_buf = buffer::RefReadBuffer::new(encrypted);
    let mut buf = [0; 4096];
    let mut write_buf = buffer::RefWriteBuffer::new(&mut buf);

    loop {
        let result = try!(decryptor.decrypt(&mut read_buf, &mut write_buf,
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

pub fn decrypt_aes_cbc(encrypted: &[u8], key: &[u8], iv: &[u8]) -> Vec<u8>{
    assert!(key.len() == iv.len());
    assert!(encrypted.len() % key.len() == 0);
    let block_size = key.len();
    let mut carry = iv;
    let mut final_result = Vec::<u8>::new();

    for i in 0 .. (encrypted.len() / block_size) {
        let next_block = &encrypted[i * block_size .. (i + 1) * block_size];
        let block = decrypt_aes_ecb(next_block, key).unwrap();
        final_result.extend(fixed_xor(&block, carry));
        carry = next_block;
    }
    final_result
}

pub fn pkcs_unpad(blocks: &mut Vec<u8>) {
    match blocks.last() {
        Some(&b) => {
            for _ in 0 .. b {
                blocks.pop();
            }
        }
        None => { }
    }
}

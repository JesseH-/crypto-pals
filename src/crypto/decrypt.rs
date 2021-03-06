extern crate crypto;

use std::collections::HashMap;

use self::crypto::{symmetriccipher, buffer, aes, blockmodes};
use self::crypto::buffer::{ReadBuffer, WriteBuffer, BufferResult};

use util::{fixed_xor};
use util::cookie::{parse_cookie};

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

pub fn pkcs_unpad(blocks: &mut Vec<u8>) -> Result<(), &'static str> {
    match blocks.last() {
        Some(&b) => {
            for _ in 0 .. b {
                match blocks.pop() {
                    Some(l) => {
                        if l != b {
                            return Err("Next byte does not match padding.")
                        }
                    }
                    None => return Err("Ran out of padding.")
                }
            }
        }
        None => { }
    }
    Ok(())
}

pub fn decrypt_profile(encrypted: &[u8], key: &[u8])
                       -> HashMap<String, String> {
                           let mut decrypted = decrypt_aes_ecb(&encrypted, &key).unwrap();
                           println!("{:?}", decrypted);
                           println!("{:?}", String::from_utf8(decrypted.clone()));
    pkcs_unpad(&mut decrypted).unwrap();
    parse_cookie(&String::from_utf8(decrypted).unwrap())
}

pub fn bitflip_decrypt(encrypted: &[u8], key: &[u8], iv: &[u8]) -> bool {
    let bytes = decrypt_aes_cbc(&encrypted, &key, &iv);
    let decrypted = String::from_utf8_lossy(&bytes);
    decrypted.contains(";admin=true;")
}

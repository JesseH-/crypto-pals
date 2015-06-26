extern crate crypto;

use self::crypto::{symmetriccipher, buffer, aes, blockmodes};
use self::crypto::buffer::{ReadBuffer, WriteBuffer, BufferResult};

pub fn decrypt_aes_ecb(encrypted: &[u8], key: &[u8]) ->
    Result<Vec<u8>, symmetriccipher::SymmetricCipherError> {
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

pub fn pkcs_pad(blocks: &mut Vec<u8>, length: usize) {
    let difference = length - (blocks.len() % length);
    for _ in 0 .. difference {
        blocks.push(difference as u8);
    }
}

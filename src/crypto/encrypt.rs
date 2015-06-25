extern crate crypto;
extern crate rand;

use self::crypto::{symmetriccipher, buffer, aes, blockmodes};
use self::crypto::buffer::{ReadBuffer, WriteBuffer, BufferResult};
use self::rand::{thread_rng, Rng};

pub fn generate_key() -> Vec<u8> {
    let mut v = vec![0u8; 16];
    thread_rng().fill_bytes(&mut v);
    v
}

pub fn encrypt_aes_ecb(encrypted: &[u8], key: &[u8]) ->
    Result<Vec<u8>, symmetriccipher::SymmetricCipherError> {
    let mut encryptor = aes::ecb_encryptor(aes::KeySize::KeySize128,
                                           key, blockmodes::PkcsPadding);

    let mut final_result = Vec::<u8>::new();
    let mut read_buf = buffer::RefReadBuffer::new(encrypted);
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

extern crate cryptopals;

use cryptopals::{utils, pretty, ecb, aes};

fn main() {
    let lines = match utils::load_file_lines("data/s1c7.txt") {
        Ok(lines) => lines,
        Err(e) => panic!(e),
    };
    let b64encoded = lines.join("");
    let ciphertext = pretty::read_b64(&b64encoded);
    // let key = "YELLOW SUBMARINE".as_bytes();
    let key: Vec<u8> = vec![0; 16];
    let plaintext = ecb::decrypt(&key, &ciphertext, aes::decrypt128);
}

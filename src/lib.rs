#![feature(exact_chunks)]

pub mod pretty;
pub mod score;
pub mod xor;
pub mod ecb;
pub mod aes;
pub mod utils;

#[cfg(test)]
mod tests {
    use pretty;
    use score::*;
    use xor;
    use aes;
    use ecb;
    use utils;

    use std::{str};

    #[test]
    fn s1c1() {
        let ref_hex = "49276d206b696c6c696e6720796f757220627261696e206c696b65206120706f69736f6e6f7573206d757368726f6f6d";
        let ref_b64 = "SSdtIGtpbGxpbmcgeW91ciBicmFpbiBsaWtlIGEgcG9pc29ub3VzIG11c2hyb29t";

        assert_eq!(pretty::write_b64(&pretty::read_hex(ref_hex)), ref_b64); // hex -> b64
        assert_eq!(pretty::write_hex(&pretty::read_b64(ref_b64)), ref_hex); // b64 -> hex
    }

    #[test]
    fn s1c2() {
        let src  = pretty::read_hex("1c0111001f010100061a024b53535009181c");
        let key  = pretty::read_hex("686974207468652062756c6c277320657965");
        let dest = "746865206b696420646f6e277420706c6179";

        match xor::fixed(&src, &key) {
            Ok(cipher) => {
                assert_eq!(pretty::write_hex(&cipher), dest);
            },
            Err(e) => panic!(e),
        }
    }

    #[test]
    fn s1c3() {
        let src = pretty::read_hex("1b37373331363f78151b7f2b783431333d78397828372d363c78373e783a393b3736");

        let scorer = EnglishASCII{};
        let (key, guess) = xor::break_single(&src, &scorer);

        assert_eq!(key, 88);
        assert_eq!(guess, "Cooking MC's like a pound of bacon");
    }

    #[test]
    fn s1c4() {
        let mut found = false;

        match utils::load_file_lines("data/s1c4.txt") {
            Ok(lines) => {
                let scorer = EnglishASCII{};

                for l in lines {
                    let src = pretty::read_hex(&l);
                    let (_, guess) = xor::break_single(&src, &scorer);
                    if guess == "Now that the party is jumping\n" {
                        found = true;
                        break;
                    }
                }
            },
            Err(e) => panic!(e),
        }

        assert!(found);
    }

    #[test]
    fn s1c5() {
        let src = "Burning 'em, if you ain't quick and nimble\nI go crazy when I hear a cymbal".as_bytes();
        let dest = "0b3637272a2b2e63622c2e69692a23693a2a3c6324202d623d63343c2a26226324272765272a282b2f20430a652e2c652a3124333a653e2b2027630c692b20283165286326302e27282f";
        let key = "ICE".as_bytes();
        
        let cipher = xor::repeating(&src, &key);

        assert_eq!(pretty::write_hex(&cipher), dest);
    }

    #[test]
    fn test_hamming() {
        let a = "this is a test".as_bytes();
        let b = "wokka wokka!!!".as_bytes();
        let d = utils::hamming(&a, &b);
        assert_eq!(d, 37);
    }

    #[test]
    fn s1c6() {
        match utils::load_file_lines("data/s1c6.txt") {
            Ok(l) => {
                let text = l.join("");
                let src = pretty::read_b64(&text);
                let key_size = xor::guess_repeating_keysize(&src, 40);

                let chunks = src.exact_chunks(key_size);
                let transposed = utils::transpose(chunks);

                let mut key = Vec::new();
                let scorer = EnglishASCII{};
                
                for block in transposed {
                    let (k, _) = xor::break_single(&block, &scorer);
                    key.push(k);
                }
                
                assert_eq!(str::from_utf8(&key), Ok("Terminator X: Bring the noise"));

                let decoded = xor::repeating(&src, &key);
                let message = str::from_utf8(&decoded).unwrap();
                
                assert!(message.starts_with("I'm back and I'm ringin' the bell"));
            },
            Err(e) => panic!(e),
        }
    }

    #[test]
    fn s1c7() {
        // openssl enc -aes-128-ecb -a -d -K '59454c4c4f57205355424d4152494e45' -nosalt -in data/s1c7.txt
        let lines = match utils::load_file_lines("data/s1c7.txt") {
            Ok(lines) => lines,
            Err(e) => panic!(e),
        };
        let b64encoded = lines.join("");
        let ciphertext = pretty::read_b64(&b64encoded);
        let key = "YELLOW SUBMARINE".as_bytes();
        let plaintext = ecb::decrypt(&key, &ciphertext, aes::decrypt128);
        let message = str::from_utf8(&plaintext).unwrap();

        assert!(message.starts_with("I'm back and I'm ringin' the bell"));
    }
}

use std::{str, u8};

pub fn read_hex(s: &str) -> Vec<u8> {
    let mut bytes = Vec::new();
    for pair in s.to_ascii_lowercase().into_bytes().chunks(2) {
        let hex_pair = match pair.len() {
            2 => [pair[0], pair[1]],
            1 => [pair[0], 0x30],
            _ => panic!("failed to chunk hex string"),
        };
        match str::from_utf8(&hex_pair) {
            Ok(s) => {
                match u8::from_str_radix(s, 16) {
                    Ok(byte) => bytes.push(byte),
                    Err(e) => panic!(e),
                }
            },
            Err(e) => panic!(e),
        };
    }
    bytes
}

pub fn write_hex(bytes: &[u8]) -> String {
    let strings: Vec<String> = bytes.into_iter().map(|b: &u8| format!("{:02X}", b)).collect();
    strings.join("").to_ascii_lowercase()
}

extern crate base64;
use self::base64::{encode, decode};

pub fn read_b64(s: &str) -> Vec<u8> {
    match decode(s) {
        Ok(bytes) => bytes,
        Err(e) => panic!(e),
    }
}

pub fn write_b64(bytes: &[u8]) -> String {
    encode(bytes)
}

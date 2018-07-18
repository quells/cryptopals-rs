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

pub fn read_b64(s: &str) -> Vec<u8> {
    let mut s = s.to_string();
    while s.len() % 4 != 0 {
        s.push('=');
    }

    let mut bytes = Vec::new();
    for quad in s.as_bytes().chunks(4) {
        let a = ascii_to_hunk(quad[0]);
        let b = ascii_to_hunk(quad[1]);
        let c = ascii_to_hunk(quad[2]);
        let d = ascii_to_hunk(quad[3]);
        let x = (a << 2) | (b >> 4); // aaaaaabb
        let y = (b << 4) | (c >> 2); // bbbbcccc
        let z = (c << 6) | d;        // ccdddddd
        bytes.push(x);
        bytes.push(y);
        bytes.push(z);
    }

    bytes
}

fn ascii_to_hunk(c: u8) -> u8 {
    if b'A' <= c && c <= b'Z' {
        return c - b'A';
    } else if b'a' <= c && c <= b'z' {
        return c - b'a' + 26;
    } else if b'0' <= c && c <= b'9' {
        return c - b'0' + 52;
    }
    match c {
        b'+' => 62,
        b'/' => 63,
        _ => 0,
    }
}

fn hunk_to_ascii(hunk: u8) -> char {
    if hunk < 26 {
        return (hunk + b'A') as char;
    }
    let mut hunk = hunk - 26;
    if hunk < 26 {
        return (hunk + b'a') as char;
    }
    hunk -= 26;
    if hunk < 10 {
        return (hunk + b'0') as char;
    }
    hunk -= 10;
    match hunk {
        0 => '+',
        1 => '/',
        _ => '=',
    }
}

pub fn write_b64(bytes: &[u8]) -> String {
    let mut bytes = bytes.to_vec();
    while bytes.len() % 3 != 0 {
        bytes.push(0);
    }
    
    let mut repr = String::new();
    for triple in bytes.chunks(3) {
        let (x, y, z) = (triple[0], triple[1], triple[2]);
        let a = x >> 2;                       // xxxxxx00
        let b = ((x & 0x03) << 4) | (y >> 4); // 000000xx yyyy0000
        let c = ((y & 0x0F) << 2) | (z >> 6); // 0000yyyy zz000000
        let d = z & 0x3F;                     // 00zzzzzz
        repr.push(hunk_to_ascii(a));
        repr.push(hunk_to_ascii(b));
        repr.push(hunk_to_ascii(c));
        repr.push(hunk_to_ascii(d));
    }
    
    repr
}

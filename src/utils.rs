extern crate std;
use std::io::prelude::*;

pub fn load_file(filename: &str) -> std::io::Result<String> {
    let mut contents = String::new();
    let mut file = std::fs::File::open(filename)?;
    file.read_to_string(&mut contents)?;
    Ok(contents)
}

pub fn load_file_lines(filename: &str) -> std::io::Result<Vec<String>> {
    match load_file(filename) {
        Ok(s) => Ok(s.split("\n").map(|s| s.to_string()).collect()),
        Err(e) => Err(e),
    }
}

pub fn count_ones(x: u8) -> usize {
    // https://prismoskills.appspot.com/lessons/Bitwise_Operators/Count_ones_in_an_integer.jsp
    let mut x = x;
    let mut count = 0;
    while x != 0 {
        x = x & (x - 1);
        count += 1;
    }
    count
}

pub fn hamming(a: &[u8], b: &[u8]) -> usize {
    a.into_iter().zip(b)
        .map(|(a, b)| a ^ b)
        .map(count_ones)
        .fold(0, |acc, x| acc + x)
}

use std::slice::ExactChunks;

pub fn transpose<T>(chunks: ExactChunks<T>) -> Vec<Vec<T>>
where T: Copy,
{
    let mut chunks = chunks;
    let mut transposed = Vec::new();

    for value in chunks.next().unwrap() {
        let mut col = Vec::new();
        col.push(*value);
        transposed.push(col);
    }
    
    loop {
        match chunks.next() {
            Some(chunk) => {
                for (idx, value) in chunk.into_iter().enumerate() {
                    let mut col = (&transposed[idx]).clone();
                    col.push(*value);
                    transposed[idx] = col;
                }
            },
            None => {
                return transposed;
            },
        }
    }
}

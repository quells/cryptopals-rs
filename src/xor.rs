use std::{str, f64};
use std::str::Utf8Error;
use score;
use utils::hamming;

pub fn fixed(src: &[u8], key: &[u8]) -> Result<Vec<u8>, &'static str> {
    if src.len() != key.len() {
        return Err("src and key are unequal lengths");
    }

    let result = key.into_iter().zip(src).map(|(k, s)| k ^ s).collect();
    Ok(result)
}

pub fn single(src: &[u8], key: u8) -> Vec<u8> {
    src.into_iter().map(|b| b ^ key).collect()
}

fn transform_for_scorer(src: &[u8], key: &u8) -> Result<String, Utf8Error> {
    let plaintext = single(src, *key);
    match str::from_utf8(&plaintext) {
        Ok(s) => Ok(s.to_string()),
        Err(e) => Err(e),
    }
}

pub fn break_single<S>(src: &[u8], scorer: &S) -> (u8, String)
where S: score::Scorer,
{
    let key_values: Vec<u8> = (1..255).collect();
    let (key, guess) = scorer.best_guess(&src, &key_values, transform_for_scorer);
    (key, guess)
}

#[allow(non_snake_case)]
pub fn repeating(src: &[u8], key: &[u8]) -> Vec<u8> {
    let K = key.len();
    src.into_iter()
        .enumerate()
        .map(|(idx, byte)| {
            let k = idx % K;
            byte ^ key[k]
        }).collect()
}

pub fn guess_repeating_keysize(src: &[u8], max_key_length: usize) -> usize {
    let mut best_key_size = 2;
    let mut best_edit_distance = f64::MAX;
    for key_size in 2 .. max_key_length {
        if 4 * key_size > src.len() { break; }
        let mut chunks = src.chunks(key_size);
        let a = chunks.next().unwrap();
        let b = chunks.next().unwrap();
        let c = chunks.next().unwrap();
        let d = chunks.next().unwrap();
        let distance = (
            hamming(a, b) + hamming(a, c) + hamming(a, d) +
            hamming(b, c) + hamming(b, d) +
            hamming(c, d)
            ) as f64;
        let norm_distance = distance / (key_size as f64);
        if norm_distance < best_edit_distance {
            best_key_size = key_size;
            best_edit_distance = norm_distance;
        }
    }
    best_key_size
}

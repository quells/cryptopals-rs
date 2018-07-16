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

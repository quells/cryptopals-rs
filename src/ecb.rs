
// F parameters: key, state, block
pub fn decrypt<F>(key: &[u8], ciphertext: &[u8], cipher: F) -> Vec<u8>
where F: Fn(&[u8], &[u8], &[u8]) -> Vec<u8>,
{
    let mut plaintext = Vec::new();
    let block_size = key.len();
    let null_state = vec![0; block_size];
    for block in ciphertext.chunks(block_size) {
        let mut block_copy = block.to_vec();
        while block_copy.len() < block_size {
            block_copy.extend_from_slice(&[0]);
        }
        let plain_block = cipher(key, &null_state, &block_copy);
        plaintext.extend_from_slice(&plain_block);
        panic!("take a break")
    }
    plaintext
}

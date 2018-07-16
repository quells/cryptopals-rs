pub trait Scorer {
    fn score(&self, c: char) -> usize;

    fn score_message(&self, msg: &str) -> usize {
        msg.chars()
            .map(|c| self.score(c))
            .fold(0, |acc, x| acc + x)
    }

    fn best_guess<K, F, E>(&self, bytes: &[u8], keys: &Vec<K>, transform: F) -> (K, String)
    where K: Copy,
    F: Fn(&[u8], &K) -> Result<String, E>
    {
        let mut max_key = match keys.split_first() {
            Some((k, _)) => *k,
            None => panic!("Scorer::best_guest requires at least 1 key to try")
        };
        let mut max_score = 0;
        let mut max_message = String::new();

        for k in keys {
            match transform(&bytes, k) {
                Ok(msg_k) => {
                    let score_k = self.score_message(&msg_k);
                    if score_k > max_score {
                        max_key = *k;
                        max_score = score_k;
                        max_message = msg_k;
                    }
                },
                Err(_) => (),
            }
        }

        (max_key, max_message)
    }
}

pub struct EnglishASCII {}
impl Scorer for EnglishASCII {
    fn score(&self, c: char) -> usize {
        let mut c = c;

        if c.is_ascii_digit() || c.is_ascii_punctuation() {
            return 1;
        }

        if c.is_ascii_whitespace() {
            return 14;
        }
        
        let mut uppercase = false;
        if c.is_alphabetic() && c.is_uppercase() {
            c = c.to_ascii_lowercase();
            uppercase = true;
        }

        let score = match c {
            'a' => 8,
            'b' => 1,
            'c' => 3,
            'd' => 4,
            'e' => 13,
            'f' => 2,
            'g' => 2,
            'h' => 6,
            'i' => 7,
            'j' => 0,
            'k' => 1,
            'l' => 4,
            'm' => 2,
            'n' => 7,
            'o' => 8,
            'p' => 2,
            'q' => 0,
            'r' => 6,
            's' => 6,
            't' => 9,
            'u' => 3,
            'v' => 1,
            'w' => 2,
            'x' => 0,
            'y' => 2,
            'z' => 0,
            _ => 0,
        };
        
        match uppercase {
            true => score / 2,
            false => score,
        }
    }
}

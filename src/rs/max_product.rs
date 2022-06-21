#[allow(dead_code)]
pub struct MaxProduct {}

#[allow(dead_code)]
impl MaxProduct {
    pub fn max_product(words: Vec<String>) -> i32 {
        let mut result = 0usize;
        let mut bits = vec![0; words.len()];
        let words_len = words.len();
        let acode = b'a';
        for i in 0..words_len {
            let word = &words[i];
            let word_len = word.len();
            let mut curbit = 0;
            for ch in word.bytes() {
                curbit |= 1 << (ch - acode);
            }

            bits[i] = curbit;
            for j in 0..i {
                let mask = curbit & bits[j];
                if mask == 0 {
                    let mul = word_len * words[j].len();
                    result = result.max(mul);
                }
            }
        }

        result as i32
    }
}

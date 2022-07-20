pub struct Solution {}
impl Solution {
    pub fn num_matching_subseq(s: String, words: Vec<String>) -> i32 {
        use std::collections::HashMap;
        let mut seen = HashMap::<&String, bool>::with_capacity(words.len());
        let mut result = 0;
        let sbytes = s.as_bytes();

        for word in words.iter() {
            if let Some(check) = seen.get(word) {
                if *check {
                    result += 1;
                }
                continue;
            }

            let mut matched = 0;
            let mut i = 0;
            let mut j = 0;
            let wbytes = word.as_bytes();
            while (i < sbytes.len()) && (j < wbytes.len()) {
                if sbytes[i] == wbytes[j] {
                    matched += 1;
                    j += 1;
                }
                i += 1;
            }

            if matched == word.len() {
                result += 1;
                seen.insert(word, true);
            } else {
                seen.insert(word, false);
            }
        }

        result
    }
}

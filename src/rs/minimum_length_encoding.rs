use std::collections::HashSet;

pub struct MinimumLengthEncoding {}
impl MinimumLengthEncoding {
    pub fn minimum_length_encoding(words: Vec<String>) -> i32 {
        let mut filter = HashSet::<String>::new();

        for word in &words {
            filter.insert(word.clone());
        }

        for word in &words {
            for i in 1..word.len() {
                let sub = &word[i..];
                filter.remove(sub);
            }
        }

        let mut result = 0usize;
        for word in filter {
            result += word.len() + 1;
        }

        result as i32
    }
}

use std::collections::{HashMap, HashSet};

struct Solution {}

impl Solution {
    pub fn most_common_word(paragraph: String, banned: Vec<String>) -> String {
        let mut banned_words = HashSet::<String>::with_capacity(banned.len());
        for b in banned.iter() {
            banned_words.insert(b.clone());
        }

        let mut result = String::new();
        let mut max_count = 0;

        let mut word_counts = HashMap::<String, i32>::new();
        let chars: Vec<char> = paragraph.chars().collect();
        let mut buffer = Vec::<char>::new();

        for (i, &ch) in chars.iter().enumerate() {
            if let Some(lower) = Self::to_lowercase(ch) {
                buffer.push(lower);
                let last_i = chars.len() - 1;
                if i < last_i {
                    continue;
                }
            }

            if buffer.is_empty() {
                continue;
            }

            let word: String = buffer.iter().collect();
            if banned_words.contains(&word) {
                buffer.clear();
                continue;
            }

            let count = word_counts.entry(word.clone()).or_insert(0);
            *count += 1;

            if *count > max_count {
                result = word;
                max_count = *count;
            }

            buffer.clear();
        }

        return result;
    }

    fn to_lowercase(ch: char) -> Option<char> {
        let code = ch as u8;
        if b'a' <= code && code <= b'z' {
            return Some(ch);
        } else if b'A' <= code && code <= b'Z' {
            let term = b'a' - b'A';
            return Some((code + term) as char);
        }

        return None;
    }
}

struct Input {
    paragraph: &'static str,
    banned: Vec<&'static str>,
}

fn main() {
    let inputs = vec![
        Input {
            paragraph: "Bob hit a ball, the hit BALL flew far after it was hit.",
            banned: vec!["hit"],
        },
        Input {
            paragraph: "a.",
            banned: vec!["hit"],
        },
    ];

    for input in inputs {
        let result = Solution::most_common_word(
            input.paragraph.to_string(),
            input.banned.iter().map(|s| s.to_string()).collect(),
        );
        println!("{result}");
    }
}

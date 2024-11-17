use std::collections::HashMap;

struct Solution {}

impl Solution {
    pub fn shortest_completing_word(license_plate: String, words: Vec<String>) -> String {
        let license_map = Self::to_map(&license_plate);
        let mut result = String::new();

        for word in words.iter() {
            let w_map = Self::to_map(word);
            let mut found = true;
            for (ch, &count) in license_map.iter() {
                if let Some(&w_count) = w_map.get(ch) {
                    if w_count < count {
                        found = false;
                        break;
                    }
                } else {
                    found = false;
                    break;
                }
            }

            if !found {
                continue;
            }

            if result.is_empty() {
                result = word.clone();
            } else if word.len() < result.len() {
                result = word.clone();
            }
        }

        return result;
    }

    fn is_letter(ch: char) -> bool {
        return ('a' <= ch && ch <= 'z') || ('A' <= ch && ch <= 'Z');
    }

    fn to_map(s: &String) -> HashMap<char, i32> {
        let mut ch_map = HashMap::<char, i32>::with_capacity(s.len());
        for ch in s.to_lowercase().chars() {
            if Self::is_letter(ch) {
                if let Some(count) = ch_map.get_mut(&ch) {
                    *count += 1;
                } else {
                    ch_map.insert(ch, 1);
                }
            }
        }

        return ch_map;
    }
}

struct Input {
    license_plate: &'static str,
    words: Vec<&'static str>,
}

fn main() {
    let inputs = vec![
        Input {
            license_plate: "1s3 PSt",
            words: vec!["step", "steps", "stripe", "stepple"],
        },
        Input {
            license_plate: "1s3 456",
            words: vec!["looks", "pest", "stew", "show"],
        },
    ];

    for input in inputs {
        let result = Solution::shortest_completing_word(
            input.license_plate.to_string(),
            input.words.iter().map(|s| s.to_string()).collect(),
        );
        println!("{result}");
    }
}

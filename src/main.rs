mod utils;

use std::collections::HashSet;
use std::iter::FromIterator;

use utils::Solution;

impl Solution {
    pub fn word_break(s: String, word_dict: Vec<String>) -> bool {
        let s = s.as_bytes();
        let words = HashSet::<&String>::from_iter(word_dict.iter());
        let mut checks = vec![false; s.len() + 1];
        checks[0] = true;

        for right in 1..=s.len() {
            for left in 0..right {
                if !checks[left] {
                    continue;
                }

                let slice = &s[left..right];
                let sub = String::from_utf8(slice.iter().cloned().collect()).unwrap();
                if words.contains(&sub) {
                    checks[right] = true;
                    break;
                }
            }
        }

        return checks[s.len()];
    }
}

fn main() {
    let inputs = [
        ("leetcode", vec!["leet", "code"]),
        ("applepenapple", vec!["apple", "pen"]),
        ("catsandog", vec!["cats", "dog", "sand", "and", "cat"]),
    ];

    for (s, word_dict) in inputs {
        let s = s.to_string();
        let word_dict = word_dict.iter().map(|s| s.to_string()).collect();
        let result = Solution::word_break(s, word_dict);
        println!("{result}");
    }
}

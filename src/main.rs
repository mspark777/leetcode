struct Solution {}

use std::collections::HashSet;

impl Solution {
    pub fn longest_nice_substring(s: String) -> String {
        let mut max_len = 0usize;
        let mut result: &[char] = &[];
        let chars = s.chars().collect::<Vec<char>>();

        for i in 0..chars.len() {
            let mut ch_set = HashSet::<char>::with_capacity(52);
            for (j, ch) in chars.iter().cloned().enumerate().skip(i) {
                ch_set.insert(ch);
                if Self::check(&ch_set) {
                    let curr_len = j + 1 - i;
                    if curr_len > max_len {
                        max_len = curr_len;
                        result = &chars[i..=j];
                    }
                }
            }
        }

        result.iter().collect()
    }

    fn check(set: &HashSet<char>) -> bool {
        for ch in set.iter() {
            let l = ch.to_ascii_lowercase();
            let u = ch.to_ascii_uppercase();
            if !(set.contains(&l) && set.contains(&u)) {
                return false;
            }
        }

        true
    }
}

struct Input {
    s: String,
}

fn main() {
    let inputs = [
        Input {
            s: "YazaAay".to_string(),
        },
        Input {
            s: "Bb".to_string(),
        },
        Input { s: "c".to_string() },
    ];

    for input in inputs {
        let result = Solution::longest_nice_substring(input.s);
        println!("{:?}", result);
    }
}

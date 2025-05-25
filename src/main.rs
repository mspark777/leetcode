struct Solution {}

impl Solution {
    pub fn longest_palindrome(words: Vec<String>) -> i32 {
        let mut counts = std::collections::HashMap::<&str, i32>::with_capacity(words.len());
        for word in words.iter() {
            let w = word.as_str();
            counts.entry(w).and_modify(|c| *c += 1).or_insert(1);
        }

        let mut result = 0;
        let mut central = false;

        for (&word, count) in counts.iter() {
            let bytes = word.as_bytes();
            let first = bytes[0];
            let second = bytes[1];
            if first == second {
                if (count & 1) == 0 {
                    result += count;
                } else {
                    result += count - 1;
                    central = true;
                }
            } else if first < second {
                let rword = String::from_utf8(vec![second, first]).unwrap();
                if let Some(rcount) = counts.get(rword.as_str()) {
                    result += 2 * count.min(rcount);
                }
            }
        }

        if central {
            result += 1;
        }

        return result * 2;
    }
}

struct Input {
    words: Vec<&'static str>,
}

fn main() {
    let inputs = vec![
        Input {
            words: ["lc", "cl", "gg"].to_vec(),
        },
        Input {
            words: ["ab", "ty", "yt", "lc", "cl", "ab"].to_vec(),
        },
        Input {
            words: ["cc", "ll", "xx"].to_vec(),
        },
    ];

    for input in inputs {
        let result =
            Solution::longest_palindrome(input.words.iter().map(|s| s.to_string()).collect());
        println!("{:?}", result);
    }
}

struct Solution {}

impl Solution {
    pub fn odd_string(words: Vec<String>) -> String {
        let mut counts = std::collections::HashMap::<String, i32>::with_capacity(words.len());
        let mut word_map = std::collections::HashMap::<String, &str>::with_capacity(words.len());

        for word in words.iter() {
            let hash = word
                .as_bytes()
                .windows(2)
                .map(|w| format!("{}_", ((w[1] as i32) - (w[0] as i32))))
                .collect::<Vec<String>>()
                .concat();

            counts
                .entry(hash.clone())
                .and_modify(|c| *c += 1)
                .or_insert(1);
            word_map.insert(hash, word.as_str());
        }

        for (hash, &count) in counts.iter() {
            if count == 1 {
                return word_map.get(hash).unwrap().to_string();
            }
        }

        return String::new();
    }
}

struct Input {
    words: Vec<String>,
}

fn main() {
    let inputs = [
        Input {
            words: ["adc", "wzy", "abc"].map(|s| s.to_string()).to_vec(),
        },
        Input {
            words: ["aaa", "bob", "ccc", "ddd"].map(|s| s.to_string()).to_vec(),
        },
        Input {
            words: ["abm", "bcn", "alm"].map(|s| s.to_string()).to_vec(),
        },
    ];

    for input in inputs {
        let result = Solution::odd_string(input.words);
        println!("{:?}", result);
    }
}

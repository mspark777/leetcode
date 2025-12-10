struct Solution {}

impl Solution {
    pub fn maximum_number_of_string_pairs(words: Vec<String>) -> i32 {
        let mut word_set = std::collections::HashSet::<String>::with_capacity(words.len() / 2);
        let mut result = 0;
        for word in words {
            if word_set.contains(&word) {
                result += 1;
            } else {
                let rev = word.chars().rev().collect::<String>();
                word_set.insert(rev);
            }
        }

        result
    }
}

struct Input {
    words: Vec<String>,
}

fn main() {
    let inputs = [
        Input {
            words: ["cd", "ac", "dc", "ca", "zz"]
                .map(|s| s.to_string())
                .to_vec(),
        },
        Input {
            words: ["ab", "ba", "cc"].map(|s| s.to_string()).to_vec(),
        },
        Input {
            words: ["aa", "ab"].map(|s| s.to_string()).to_vec(),
        },
    ];

    for input in inputs {
        let result = Solution::maximum_number_of_string_pairs(input.words);
        println!("{:?}", result);
    }
}

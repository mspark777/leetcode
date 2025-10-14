struct Solution {}

impl Solution {
    pub fn count_prefixes(words: Vec<String>, s: String) -> i32 {
        let mut result = 0;
        for word in words.iter() {
            if s.starts_with(word) {
                result += 1;
            }
        }

        result
    }
}

struct Input {
    words: Vec<String>,
    s: String,
}

fn main() {
    let inputs = [
        Input {
            words: ["a", "b", "c", "ab", "bc", "abc"]
                .map(|s| s.to_string())
                .to_vec(),
            s: "abc".to_string(),
        },
        Input {
            words: ["a", "a"].map(|s| s.to_string()).to_vec(),
            s: "aa".to_string(),
        },
    ];

    for input in inputs {
        let result = Solution::count_prefixes(input.words, input.s);
        println!("{:?}", result);
    }
}

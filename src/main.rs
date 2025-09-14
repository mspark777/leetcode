struct Solution {}

impl Solution {
    pub fn num_of_strings(patterns: Vec<String>, word: String) -> i32 {
        patterns.iter().fold(
            0,
            |acc, pattern| {
                if word.contains(pattern) { acc + 1 } else { acc }
            },
        )
    }
}

struct Input {
    patterns: Vec<String>,
    word: String,
}

fn main() {
    let inputs = [
        Input {
            patterns: ["a", "abc", "bc", "d"].map(|s| s.to_string()).to_vec(),
            word: "abc".to_string(),
        },
        Input {
            patterns: ["a", "b", "c"].map(|s| s.to_string()).to_vec(),
            word: "aaaaabbbbb".to_string(),
        },
        Input {
            patterns: ["a", "a", "a"].map(|s| s.to_string()).to_vec(),
            word: "ab".to_string(),
        },
    ];

    for input in inputs {
        let result = Solution::num_of_strings(input.patterns, input.word);
        println!("{:?}", result);
    }
}

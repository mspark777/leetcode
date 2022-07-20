mod solution;

use solution::Solution;

struct Input {
    s: String,
    words: Vec<String>,
}

fn main() {
    let inputs: Vec<Input> = vec![
        Input {
            s: "abcde".to_string(),
            words: ["a", "bb", "acd", "ace"]
                .iter()
                .map(|s| String::from(*s))
                .collect(),
        },
        Input {
            s: "dsahjpjauf".to_string(),
            words: ["ahjpjau", "ja", "ahbwzgqnuk", "tnmlanowax"]
                .iter()
                .map(|s| String::from(*s))
                .collect(),
        },
    ];

    for input in inputs {
        let result = Solution::num_matching_subseq(input.s, input.words);
        println!("{result:?}");
    }
}

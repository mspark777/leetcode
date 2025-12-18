struct Solution;

impl Solution {
    pub fn split_words_by_separator(words: Vec<String>, separator: char) -> Vec<String> {
        words
            .iter()
            .flat_map(|word| word.split(separator))
            .filter(|&s| !s.is_empty())
            .map(|s| s.to_string())
            .collect()
    }
}

struct Input {
    words: Vec<String>,
    separator: char,
}

fn main() {
    let inputs = [
        Input {
            words: ["one.two.three", "four.five", "six"]
                .map(|s| s.to_string())
                .to_vec(),
            separator: '.',
        },
        Input {
            words: ["$easy$", "$problem$"].map(|s| s.to_string()).to_vec(),
            separator: '$',
        },
        Input {
            words: ["|||"].map(|s| s.to_string()).to_vec(),
            separator: '|',
        },
    ];

    for input in inputs {
        let result = Solution::split_words_by_separator(input.words, input.separator);
        println!("{:?}", result);
    }
}

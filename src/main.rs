struct Solution {}

impl Solution {
    pub fn can_be_typed_words(text: String, broken_letters: String) -> i32 {
        let brokens = std::collections::HashSet::<char>::from_iter(broken_letters.chars());
        text.split_whitespace()
            .filter(|word| word.chars().all(|c| !brokens.contains(&c)))
            .count() as i32
    }
}

struct Input {
    text: String,
    broken_letters: String,
}

fn main() {
    let inputs = [
        Input {
            text: "hello world".to_string(),
            broken_letters: "ad".to_string(),
        },
        Input {
            text: "leet code".to_string(),
            broken_letters: "lt".to_string(),
        },
        Input {
            text: "leet code".to_string(),
            broken_letters: "e".to_string(),
        },
    ];

    for input in inputs {
        let result = Solution::can_be_typed_words(input.text, input.broken_letters);
        println!("{:?}", result);
    }
}

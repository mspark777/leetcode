struct Solution {}

impl Solution {
    pub fn is_valid(word: String) -> bool {
        let mut has_vowel = false;
        let mut has_consonant = false;
        let mut has_invalid = false;
        let mut len = 0usize;

        for ch in word.chars() {
            len += 1;
            if ch.is_ascii_alphabetic() {
                let ch = ch.to_ascii_lowercase();
                if "aeiou".contains(ch) {
                    has_vowel = true;
                } else {
                    has_consonant = true;
                }
            } else if !ch.is_ascii_digit() {
                has_invalid = true;
                break;
            }
        }

        has_vowel && has_consonant && !has_invalid && len >= 3
    }
}

struct Input {
    word: &'static str,
}

fn main() {
    let inputs = vec![
        Input { word: "234Adas" },
        Input { word: "b3" },
        Input { word: "a3$e" },
    ];

    for input in inputs {
        let result = Solution::is_valid(input.word.to_string());
        println!("{:?}", result);
    }
}

struct Solution;

impl Solution {
    pub fn trim_trailing_vowels(s: String) -> String {
        let chars = s.chars().collect::<Vec<char>>();
        let mut right = -1;
        for (i, ch) in chars.into_iter().enumerate().rev() {
            if !Self::is_vowel(ch) {
                right = i as i32;
                break;
            }
        }

        match right {
            -1 => String::new(),
            _ => s.chars().take((right + 1) as usize).collect(),
        }
    }

    fn is_vowel(ch: char) -> bool {
        matches!(ch, 'a' | 'e' | 'i' | 'o' | 'u')
    }
}

struct Input {
    s: String,
}

fn main() {
    let inputs = [
        Input {
            s: "idea".to_string(),
        },
        Input {
            s: "day".to_string(),
        },
    ];

    for input in inputs {
        let result = Solution::trim_trailing_vowels(input.s);
        println!("{:?}", result);
    }
}

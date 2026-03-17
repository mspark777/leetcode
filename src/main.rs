struct Solution;

impl Solution {
    pub fn generate_tag(caption: String) -> String {
        const LOWER: i32 = 0;
        const UPPER: i32 = 1;
        const MAX_LEN: usize = 100;

        let mut result = String::with_capacity(caption.len().min(MAX_LEN));
        result.push('#');

        let mut current = LOWER;
        for ch in caption.trim().chars() {
            if ch == ' ' {
                current = UPPER;
            } else if current == LOWER {
                result.push(ch.to_ascii_lowercase());
            } else {
                current = LOWER;
                result.push(ch.to_ascii_uppercase());
            }

            if result.len() >= MAX_LEN {
                break;
            }
        }

        result
    }
}

struct Input {
    caption: String,
}

fn main() {
    let inputs = [Input {
        caption: "Leetcode daily streak achieved".to_string(),
    }, Input {
        caption: "can I Go There".to_string(),
    }, Input {
        caption: "hhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhh".to_string()
    }];

    for input in inputs {
        let result = Solution::generate_tag(input.caption);
        println!("{:?}", result);
    }
}

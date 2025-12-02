struct Solution {}

impl Solution {
    pub fn minimized_string_length(s: String) -> i32 {
        std::collections::HashSet::<char>::from_iter(s.chars()).len() as i32
    }
}

struct Input {
    s: String,
}

fn main() {
    let inputs = [
        Input {
            s: "aaabc".to_string(),
        },
        Input {
            s: "cbbd".to_string(),
        },
        Input {
            s: "baadccad".to_string(),
        },
    ];

    for input in inputs {
        let result = Solution::minimized_string_length(input.s);
        println!("{:?}", result);
    }
}

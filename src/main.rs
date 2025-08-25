struct Solution {}

impl Solution {
    pub fn max_length_between_equal_characters(s: String) -> i32 {
        let mut first_positions = std::collections::HashMap::<char, usize>::new();
        let mut result = -1;

        for (i, c) in s.chars().enumerate() {
            if let Some(&idx) = first_positions.get(&c) {
                result = result.max((i - idx) as i32 - 1);
            } else {
                first_positions.insert(c, i);
            }
        }

        result
    }
}

struct Input {
    s: String,
}

fn main() {
    let inputs = [
        Input {
            s: "aa".to_string(),
        },
        Input {
            s: "abca".to_string(),
        },
        Input {
            s: "cbzxy".to_string(),
        },
    ];

    for input in inputs {
        let result = Solution::max_length_between_equal_characters(input.s);
        println!("{:?}", result);
    }
}

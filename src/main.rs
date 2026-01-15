struct Solution;

impl Solution {
    pub fn count_key_changes(s: String) -> i32 {
        let mut result = 0;

        for (left, right) in s.chars().zip(s.chars().skip(1)) {
            let l = left.to_ascii_lowercase();
            let r = right.to_ascii_lowercase();
            if l != r {
                result += 1;
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
            s: "aAbBcC".to_string(),
        },
        Input {
            s: "AaAaAaaA".to_string(),
        },
    ];

    for input in inputs {
        let result = Solution::count_key_changes(input.s);
        println!("{:?}", result);
    }
}

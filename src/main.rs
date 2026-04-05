struct Solution;

impl Solution {
    pub fn largest_even(s: String) -> String {
        let len = s.len();
        let mut skip = 0usize;

        for ch in s.chars().rev() {
            if ch == '2' {
                break;
            } else {
                skip += 1;
            }
        }

        s.chars().take(len - skip).collect()
    }
}

struct Input {
    s: String,
}

fn main() {
    let inputs = [
        Input {
            s: "1112".to_string(),
        },
        Input {
            s: "221".to_string(),
        },
        Input { s: "1".to_string() },
        Input {
            s: "11".to_string(),
        },
    ];

    for input in inputs {
        let result = Solution::largest_even(input.s);
        println!("{:?}", result);
    }
}

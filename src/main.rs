struct Solution;

impl Solution {
    pub fn residue_prefixes(s: String) -> i32 {
        use std::collections::HashSet;

        let mut counts = HashSet::<char>::new();
        let mut result = 0;
        for (i, ch) in s.chars().enumerate() {
            counts.insert(ch);
            if counts.len() == ((i + 1) % 3) {
                result += 1;
            }

            if counts.len() > 2 {
                break;
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
            s: "abc".to_string(),
        },
        Input {
            s: "dd".to_string(),
        },
        Input {
            s: "bob".to_string(),
        },
    ];

    for input in inputs {
        let result = Solution::residue_prefixes(input.s);
        println!("{:?}", result);
    }
}

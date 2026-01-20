struct Solution;

impl Solution {
    pub fn is_substring_present(s: String) -> bool {
        use std::collections::HashSet;
        let mut seen = HashSet::<u32>::with_capacity(100);

        for (left, right) in s.chars().zip(s.chars().skip(1)) {
            let l = (left as u32) & 0x1f;
            let r = (right as u32) & 0x1f;
            seen.insert(l << 5 | r);
            if seen.contains(&(l | r << 5)) {
                return true;
            }
        }

        false
    }
}

struct Input {
    s: String,
}

fn main() {
    let inputs = [
        Input {
            s: "leetcode".to_string(),
        },
        Input {
            s: "abcba".to_string(),
        },
        Input {
            s: "abcd".to_string(),
        },
    ];

    for input in inputs {
        let result = Solution::is_substring_present(input.s);
        println!("{:?}", result);
    }
}

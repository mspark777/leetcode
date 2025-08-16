struct Solution {}

impl Solution {
    pub fn max_power(s: String) -> i32 {
        let mut result = 1usize;
        let mut len = 1usize;
        let mut prev = s.chars().nth(0).unwrap();

        for ch in s.chars().skip(1) {
            if prev == ch {
                len += 1;
            } else {
                result = result.max(len);
                len = 1;
                prev = ch;
            }
        }

        result.max(len) as i32
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
            s: "abbcccddddeeeeedcba".to_string(),
        },
        Input {
            s: "cc".to_string(),
        },
    ];

    for input in inputs {
        let result = Solution::max_power(input.s);
        println!("{:?}", result);
    }
}

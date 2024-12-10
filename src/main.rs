struct Solution {}

impl Solution {
    pub fn maximum_length(s: String) -> i32 {
        let mut counts = std::collections::HashMap::<(char, usize), usize>::new();
        let chars = s.chars().collect::<Vec<char>>();
        for start in 0..chars.len() {
            let char = chars[start];
            let mut substring_length = 0usize;
            for end in start..chars.len() {
                if char == chars[end] {
                    substring_length += 1;
                    *counts.entry((char, substring_length)).or_insert(0) += 1;
                } else {
                    break;
                }
            }
        }

        let mut result = 0usize;
        for (&key, &value) in counts.iter() {
            let len = key.1;
            if value >= 3 && len > result {
                result = len;
            }
        }

        return if result == 0 { -1 } else { result as i32 };
    }
}

struct Input {
    s: &'static str,
}

fn main() {
    let inputs = vec![
        Input { s: "aaaa" },
        Input { s: "abcdef" },
        Input { s: "abcaba" },
    ];

    for input in inputs {
        let result = Solution::maximum_length(input.s.to_string());
        println!("{result}");
    }
}

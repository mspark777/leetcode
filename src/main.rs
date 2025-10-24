struct Solution {}

impl Solution {
    pub fn repeated_character(s: String) -> char {
        let mut checks = vec![false; 26];

        for c in s.chars() {
            let code = c as usize;
            let a = 'a' as usize;
            let idx = code - a;
            if checks[idx] {
                return c;
            }

            checks[idx] = true
        }

        return ' ';
    }
}

struct Input {
    s: String,
}

fn main() {
    let inputs = [
        Input {
            s: "abccbaacz".to_string(),
        },
        Input {
            s: "abcdd".to_string(),
        },
    ];

    for input in inputs {
        let result = Solution::repeated_character(input.s);
        println!("{:?}", result);
    }
}

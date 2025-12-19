struct Solution;

impl Solution {
    pub fn final_string(s: String) -> String {
        let mut chars = Vec::<char>::with_capacity(s.len());
        for ch in s.chars() {
            if ch == 'i' {
                chars.reverse();
            } else {
                chars.push(ch);
            }
        }

        chars.iter().collect()
    }
}

struct Input {
    s: String,
}

fn main() {
    let inputs = [
        Input {
            s: "string".to_string(),
        },
        Input {
            s: "poiinter".to_string(),
        },
    ];

    for input in inputs {
        let result = Solution::final_string(input.s);
        println!("{:?}", result);
    }
}

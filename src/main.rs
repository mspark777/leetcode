struct Solution {}

impl Solution {
    pub fn check_string(s: String) -> bool {
        s.find("ba").is_none()
    }
}

struct Input {
    s: String,
}

fn main() {
    let inputs = [
        Input {
            s: "aaabbb".to_string(),
        },
        Input {
            s: "abab".to_string(),
        },
        Input {
            s: "bbb".to_string(),
        },
    ];

    for input in inputs {
        let result = Solution::check_string(input.s);
        println!("{:?}", result);
    }
}

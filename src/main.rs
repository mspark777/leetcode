struct Solution;

impl Solution {
    pub fn has_match(s: String, p: String) -> bool {
        let (left, right) = p.split_once("*").unwrap();

        match (s.find(left), s.rfind(right)) {
            (Some(i), Some(j)) => i + left.len() <= j,
            _ => false,
        }
    }
}

struct Input {
    s: String,
    p: String,
}

fn main() {
    let inputs = [
        Input {
            s: "leetcode".to_string(),
            p: "ee*e".to_string(),
        },
        Input {
            s: "car".to_string(),
            p: "s*v".to_string(),
        },
        Input {
            s: "luck".to_string(),
            p: "u*".to_string(),
        },
    ];

    for input in inputs {
        let result = Solution::has_match(input.s, input.p);
        println!("{:?}", result);
    }
}

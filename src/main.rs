struct Solution;

impl Solution {
    pub fn score_of_string(s: String) -> i32 {
        let mut result = 0;
        for (left, right) in s.chars().zip(s.chars().skip(1)) {
            let l = left as i32;
            let r = right as i32;
            result += (l - r).abs();
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
            s: "hello".to_string(),
        },
        Input {
            s: "zaz".to_string(),
        },
    ];

    for input in inputs {
        let result = Solution::score_of_string(input.s);
        println!("{:?}", result);
    }
}

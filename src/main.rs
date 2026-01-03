struct Solution;

impl Solution {
    pub fn find_minimum_operations(s1: String, s2: String, s3: String) -> i32 {
        let count = s1
            .chars()
            .zip(s2.chars())
            .zip(s3.chars())
            .take_while(|((c1, c2), c3)| c1 == c2 && c1 == c3)
            .count();

        match count {
            1.. => (s1.len() + s2.len() + s3.len() - 3 * count) as i32,
            _ => -1,
        }
    }
}

struct Input {
    s1: String,
    s2: String,
    s3: String,
}

fn main() {
    let inputs = [
        Input {
            s1: "abc".to_string(),
            s2: "abb".to_string(),
            s3: "ab".to_string(),
        },
        Input {
            s1: "dac".to_string(),
            s2: "bac".to_string(),
            s3: "cac".to_string(),
        },
    ];

    for input in inputs {
        let result = Solution::find_minimum_operations(input.s1, input.s2, input.s3);
        println!("{:?}", result);
    }
}

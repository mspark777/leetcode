struct Solution {}

impl Solution {
    pub fn digit_count(num: String) -> bool {
        let mut counts = vec![0usize; 10];
        for c in num.chars() {
            let idx = c as usize - '0' as usize;
            counts[idx] += 1;
        }

        for (i, c) in num.chars().enumerate() {
            let count = c as usize - '0' as usize;
            if counts[i] != count {
                return false;
            }
        }

        return true;
    }
}

struct Input {
    s: String,
}

fn main() {
    let inputs = [
        Input {
            s: "1210".to_string(),
        },
        Input {
            s: "030".to_string(),
        },
    ];

    for input in inputs {
        let result = Solution::digit_count(input.s);
        println!("{:?}", result);
    }
}

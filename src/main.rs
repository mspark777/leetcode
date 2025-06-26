struct Solution {}

impl Solution {
    pub fn longest_subsequence(s: String, k: i32) -> i32 {
        let mut sum = 0;
        let mut result = 0;
        let bits = (k as f64).log2() as usize + 1;
        for (i, ch) in s.chars().rev().enumerate() {
            if ch == '1' {
                if i < bits && sum + (1 << i) <= k {
                    sum += 1 << i;
                    result += 1;
                }
            } else {
                result += 1;
            }
        }

        result
    }
}

struct Input {
    s: &'static str,
    k: i32,
}

fn main() {
    let inputs = vec![
        Input { s: "1001010", k: 5 },
        Input {
            s: "00101001",
            k: 1,
        },
    ];

    for input in inputs {
        let result = Solution::longest_subsequence(input.s.to_string(), input.k);
        println!("{:?}", result);
    }
}

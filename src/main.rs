struct Solution {}

impl Solution {
    pub fn digit_sum(s: String, k: i32) -> String {
        let k = k as usize;
        let mut nums = s.chars().collect::<Vec<char>>();

        while nums.len() > k {
            let mut sums = Vec::<char>::with_capacity((nums.len() / k) + 1);
            for chunk in nums.chunks(k) {
                let sum = chunk
                    .iter()
                    .fold(0u32, |acc, x| acc + x.to_digit(10).unwrap());

                for ch in sum.to_string().chars() {
                    sums.push(ch);
                }
            }

            nums = sums;
        }

        nums.iter().collect()
    }
}

struct Input {
    s: String,
    k: i32,
}

fn main() {
    let inputs = [
        Input {
            s: "11111222223".to_string(),
            k: 3,
        },
        Input {
            s: "00000000".to_string(),
            k: 3,
        },
    ];

    for input in inputs {
        let result = Solution::digit_sum(input.s, input.k);
        println!("{:?}", result);
    }
}

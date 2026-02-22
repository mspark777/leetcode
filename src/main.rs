struct Solution;

impl Solution {
    pub fn count_k_constraint_substrings(s: String, k: i32) -> i32 {
        let s = s.chars().collect::<Vec<char>>();
        let n = s.len();
        let mut left = 0usize;
        let mut right = 0usize;
        let mut ones = 0;
        let mut zeros = 0;
        let mut result = 0;

        while right < n {
            if s[right] == '0' {
                zeros += 1;
            } else {
                ones += 1;
            }

            while (zeros > k) && (ones > k) {
                if s[left] == '0' {
                    zeros -= 1;
                } else {
                    ones -= 1;
                }

                left += 1;
            }

            result += (right + 1 - left) as i32;
            right += 1;
        }

        result
    }
}

struct Input {
    s: String,
    k: i32,
}

fn main() {
    let inputs = [
        Input {
            s: "10101".to_string(),
            k: 1,
        },
        Input {
            s: "1010101".to_string(),
            k: 2,
        },
        Input {
            s: "11111".to_string(),
            k: 1,
        },
    ];

    for input in inputs {
        let result = Solution::count_k_constraint_substrings(input.s, input.k);
        println!("{:?}", result);
    }
}

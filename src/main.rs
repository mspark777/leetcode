struct Solution;

impl Solution {
    pub fn total_numbers(digits: Vec<i32>) -> i32 {
        let mut frequencies = [0; 10];

        for digit in digits.iter().copied() {
            let d = digit as usize;
            frequencies[d] += 1;
        }

        let mut zero = 0;
        let mut even = 0;
        let mut all = 0;

        for (i, frequency) in frequencies.iter().copied().enumerate() {
            if frequency < 1 {
                continue;
            }

            if i == 0 {
                zero += 1;
            }

            if (i & 1) == 0 {
                even += 1;
            }

            all += 1;
        }

        let mut result = even * (all - 1) * (all - 2);
        if zero == 1 {
            result -= (even - 1) * (all - 2);
        }

        for (i, frequency) in frequencies.iter().copied().enumerate() {
            if frequency < 2 {
                continue;
            }

            if i == 0 {
                result += all - 1;
            } else if (i & 1) == 1 {
                result += even;
            } else {
                result += 3 * (even - 1) - zero;
                result += 2 * (all - even);
            }
        }

        for frequency in frequencies.iter().skip(2).step_by(2).copied() {
            if frequency >= 3 {
                result += 1;
            }
        }

        result
    }
}

struct Input {
    digits: Vec<i32>,
}

fn main() {
    let inputs = [
        Input {
            digits: [1, 2, 3, 4].to_vec(),
        },
        Input {
            digits: [0, 2, 2].to_vec(),
        },
        Input {
            digits: [6, 6, 6].to_vec(),
        },
        Input {
            digits: [1, 3, 5].to_vec(),
        },
    ];

    for input in inputs {
        let result = Solution::total_numbers(input.digits);
        println!("{:?}", result);
    }
}

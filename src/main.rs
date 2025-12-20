struct Solution;

impl Solution {
    pub fn max_sum(nums: Vec<i32>) -> i32 {
        let mut result = -1;
        let mut max_num = [0; 10];
        for num in nums {
            let max_digit = Self::max_digit(num);
            if max_num[max_digit] != 0 {
                result = result.max(max_num[max_digit] + num);
            }

            max_num[max_digit] = max_num[max_digit].max(num);
        }

        result
    }

    fn max_digit(n: i32) -> usize {
        let mut n = n as usize;
        let mut m = 0usize;

        while n > 0 {
            m = m.max(n % 10);
            n /= 10;
        }

        m
    }
}

struct Input {
    nums: Vec<i32>,
}

fn main() {
    let inputs = [
        Input {
            nums: [112, 131, 411].to_vec(),
        },
        Input {
            nums: [2536, 1613, 3366, 162].to_vec(),
        },
        Input {
            nums: [51, 71, 17, 24, 42].to_vec(),
        },
    ];

    for input in inputs {
        let result = Solution::max_sum(input.nums);
        println!("{:?}", result);
    }
}

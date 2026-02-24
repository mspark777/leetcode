struct Solution;

impl Solution {
    pub fn get_sneaky_numbers(nums: Vec<i32>) -> Vec<i32> {
        let mut sum = 0.0;
        let mut squared_sum = 0.0;
        for n in nums.iter().copied() {
            let f = n as f64;
            sum += f;
            squared_sum += f * f;
        }

        let n = nums.len() as i32 - 2;
        let sum2 = sum - (n * (n - 1) / 2) as f64;
        let squared_sum2 = squared_sum - (n * (n - 1) * (2 * n - 1) / 6) as f64;
        let t = (2.0 * squared_sum2 - sum2 * sum2).sqrt();
        let x1 = (sum2 - t) / 2.0;
        let x2 = (sum2 + t) / 2.0;
        vec![x1 as i32, x2 as i32]
    }
}

struct Input {
    nums: Vec<i32>,
}

fn main() {
    let inputs = [
        Input {
            nums: [0, 1, 1, 0].to_vec(),
        },
        Input {
            nums: [0, 3, 2, 1, 3, 2].to_vec(),
        },
        Input {
            nums: [7, 1, 5, 4, 3, 4, 6, 0, 9, 5, 8, 2].to_vec(),
        },
    ];

    for input in inputs {
        let result = Solution::get_sneaky_numbers(input.nums);
        println!("{:?}", result);
    }
}

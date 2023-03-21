mod utils;
use utils::Solution;

impl Solution {
    pub fn zero_filled_subarray(nums: Vec<i32>) -> i64 {
        let mut result = 0i64;
        let mut sub = 0i64;

        for &num in nums.iter() {
            if num == 0 {
                sub += 1;
            } else {
                sub = 0;
            }

            result += sub;
        }

        return result;
    }
}

fn main() {
    let inputs = [
        vec![1, 3, 0, 0, 2, 0, 0, 4],
        vec![0, 0, 0, 2, 0, 0],
        vec![2, 10, 2019],
    ];

    for nums in inputs {
        let result = Solution::zero_filled_subarray(nums);
        println!("{result}")
    }
}

mod utils;

use utils::Solution;

impl Solution {
    pub fn minimize_array_value(nums: Vec<i32>) -> i32 {
        let mut result = 0usize;
        let mut prefix_sum = 0usize;

        for (i, &num) in nums.iter().enumerate() {
            prefix_sum += num as usize;
            result = result.max((prefix_sum + i) / (i + 1));
        }

        return result as i32;
    }
}

fn main() {
    let inputs = [vec![3, 7, 1, 6], vec![10, 1]];

    for nums in inputs {
        let result = Solution::minimize_array_value(nums);
        println!("{result}");
    }
}

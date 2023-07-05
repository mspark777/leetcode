mod utils;

use utils::Solution;

impl Solution {
    pub fn longest_subarray(nums: Vec<i32>) -> i32 {
        let mut zero_count = 0;
        let mut result = 0usize;
        let mut start = 0;

        for (i, &num) in nums.iter().enumerate() {
            if num == 0 {
                zero_count += 1;
            }

            while zero_count > 1 {
                if nums[start] == 0 {
                    zero_count -= 1;
                }
                start += 1;
            }

            result = result.max(i - start);
        }

        return result as i32;
    }
}

fn main() {
    let inputs = [
        vec![1, 1, 0, 1],
        vec![0, 1, 1, 1, 0, 1, 1, 0, 1],
        vec![1, 1, 1],
    ];

    for nums in inputs {
        let result = Solution::longest_subarray(nums);
        println!("{result}");
    }
}

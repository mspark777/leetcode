mod utils;

use utils::Solution;

impl Solution {
    pub fn first_missing_positive(nums: Vec<i32>) -> i32 {
        let mut nums = nums;
        let len32 = nums.len() as i32;
        for i in 0..nums.len() {
            while (nums[i] > 0) && (nums[i] <= len32) && (nums[(nums[i] - 1) as usize] != nums[i]) {
                let temp = nums[i];
                let j = (temp - 1) as usize;
                nums[i] = nums[j];
                nums[j] = temp;
            }
        }

        for (i, &num) in nums.iter().enumerate() {
            let j = i as i32;
            if num != (j + 1) {
                return j + 1;
            }
        }

        return len32 + 1;
    }
}

fn main() {
    let inputs = [vec![1, 2, 0], vec![3, 4, -1, 1], vec![7, 8, 9, 11, 12]];

    for nums in inputs {
        let result = Solution::first_missing_positive(nums);
        println!("{result}");
    }
}

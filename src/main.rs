mod utils;

use utils::Solution;

impl Solution {
    pub fn min_sub_array_len(target: i32, nums: Vec<i32>) -> i32 {
        let mut left = 0usize;
        let mut sum = 0;
        let mut result = i32::MAX;

        for (right, &num) in nums.iter().enumerate() {
            sum += num;
            while sum >= target {
                let l = right - left + 1;
                result = result.min(l as i32);
                sum -= nums[left];
                left += 1;
            }
        }

        return if result < i32::MAX { result } else { 0 };
    }
}

fn main() {
    let inputs = [
        (7, vec![2, 3, 1, 2, 4, 3]),
        (4, vec![1, 4, 4]),
        (11, vec![1, 1, 1, 1, 1, 1, 1, 1]),
    ];

    for (target, nums) in inputs {
        let result = Solution::min_sub_array_len(target, nums);
        println!("{result}");
    }
}

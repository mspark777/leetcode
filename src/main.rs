mod utils;

use utils::Solution;

impl Solution {
    pub fn minimize_max(nums: Vec<i32>, p: i32) -> i32 {
        let mut nums = nums;
        nums.sort_unstable();

        let mut left = 0;
        let mut right = nums[nums.len() - 1] - nums[0];

        while left < right {
            let mid = (left + right) / 2;
            if Self::count_valid_pairs(&nums, mid) >= p {
                right = mid;
            } else {
                left = mid + 1;
            }
        }

        return left;
    }

    fn count_valid_pairs(nums: &Vec<i32>, threshold: i32) -> i32 {
        let mut index = 1usize;
        let mut count = 0;
        while index < nums.len() {
            let first = nums[index - 1];
            let second = nums[index];
            let diff = second - first;
            if diff <= threshold {
                count += 1;
                index += 1;
            }

            index += 1;
        }

        return count;
    }
}

fn main() {
    let inputs = [(vec![10, 1, 2, 7, 1, 3], 2), (vec![4, 2, 1, 2], 1)];

    for (nums, p) in inputs {
        let result = Solution::minimize_max(nums, p);
        println!("{result}");
    }
}

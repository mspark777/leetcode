pub struct Solution {}

impl Solution {
    pub fn min_moves2(nums: Vec<i32>) -> i32 {
        let mut nums = nums;
        nums.sort_unstable();

        let mid = nums[nums.len() / 2];
        nums.iter().fold(0, |acc, cur| acc + (mid - cur).abs())
    }
}

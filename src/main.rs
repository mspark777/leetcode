mod utils;

use utils::Solution;

impl Solution {
    pub fn predict_the_winner(nums: Vec<i32>) -> bool {
        let n = nums.len();
        let mut dp = nums.clone();

        for diff in 1..n {
            for left in 0..(n - diff) {
                let right = left + diff;
                let lnum = nums[left];
                let rnum = nums[right];
                let ldp = dp[left];
                let rdp = dp[left + 1];
                dp[left] = (lnum - rdp).max(rnum - ldp);
            }
        }

        return dp[0] >= 0;
    }
}

fn main() {
    let inputs = [vec![1, 5, 2], vec![1, 5, 233, 7]];

    for nums in inputs {
        let result = Solution::predict_the_winner(nums);
        println!("{result}");
    }
}

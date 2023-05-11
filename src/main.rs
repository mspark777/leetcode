mod utils;

use utils::Solution;

impl Solution {
    pub fn max_uncrossed_lines(nums1: Vec<i32>, nums2: Vec<i32>) -> i32 {
        let len1 = nums1.len();
        let len2 = nums2.len();

        let mut dp = vec![0; len2 + 1];
        let mut dp_prev = vec![0; len2 + 1];

        for i in 1..=len1 {
            for j in 1..=len2 {
                if nums1[i - 1] == nums2[j - 1] {
                    dp[j] = 1 + dp_prev[j - 1];
                } else {
                    dp[j] = dp[j - 1].max(dp_prev[j]);
                }
            }

            dp_prev = dp.clone();
        }

        return dp[len2];
    }
}

fn main() {
    let inputs = [
        (vec![1, 4, 2], vec![1, 2, 4]),
        (vec![2, 5, 1, 2, 5], vec![10, 5, 2, 1, 5, 2]),
        (vec![1, 3, 7, 1, 7, 5], vec![1, 9, 2, 5, 1]),
        (vec![3, 2], vec![2, 2, 2, 3]),
    ];

    for (nums1, nums2) in inputs {
        let result = Solution::max_uncrossed_lines(nums1, nums2);
        println!("{result}");
    }
}

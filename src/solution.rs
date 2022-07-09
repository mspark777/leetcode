use std::collections::VecDeque;

pub struct Solution {}

impl Solution {
    pub fn max_result(nums: Vec<i32>, k: i32) -> i32 {
        let nums_len = nums.len();
        let mut dp = vec![nums[0]; nums_len];
        let mut q = VecDeque::<usize>::with_capacity(nums_len);
        q.push_front(0);

        for i in 1..nums_len {
            if let Some(f) = q.front() {
                let f = *f as i32;
                let s = (i as i32) - k;
                if f < s {
                    q.pop_front();
                }
            }

            dp[i] = nums[i] + dp[*q.front().unwrap()];
            while !q.is_empty() && (dp[*q.back().unwrap()] <= dp[i]) {
                q.pop_back();
            }

            q.push_back(i);
        }

        dp[nums_len - 1]
    }
}

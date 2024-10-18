struct Solution {}

impl Solution {
    pub fn count_max_or_subsets(nums: Vec<i32>) -> i32 {
        let mut max_idx = 0;
        let mut dp = vec![0; 1 << 17];

        dp[0] = 1;

        for &num in nums.iter() {
            for i in (0..=max_idx).rev() {
                let n = num as usize;
                let idx = i as usize;
                dp[idx | n] += dp[idx];
            }

            max_idx |= num;
        }

        return dp[max_idx as usize];
    }
}

fn main() {
    let inputs = vec![vec![3, 1], vec![2, 2, 2], vec![3, 2, 1, 5]];

    for input in inputs {
        let result = Solution::count_max_or_subsets(input);
        println!("{result}");
    }
}

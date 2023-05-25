mod utils;

use utils::Solution;

impl Solution {
    pub fn new21_game(n: i32, k: i32, max_pts: i32) -> f64 {
        if k == 0 {
            return 1.0;
        } else if n >= (k + max_pts) {
            return 1.0;
        }

        let mut dp = vec![0.0; (n + 1) as usize];
        dp[0] = 1.0;
        let mut sum = 1.0;
        let mut result = 0.0;

        for i in 1..=(n as usize) {
            dp[i] = sum / (max_pts as f64);
            if i < (k as usize) {
                sum += dp[i];
            } else {
                result += dp[i];
            }

            let m = max_pts as usize;
            if i >= m {
                sum -= dp[i - m];
            }
        }

        return result;
    }
}

fn main() {
    let inputs = [(10, 1, 10), (6, 1, 10), (21, 17, 10)];

    for (n, k, max_pts) in inputs {
        let result = Solution::new21_game(n, k, max_pts);
        println!("{result}");
    }
}

mod utils;

use utils::Solution;

impl Solution {
    pub fn strange_printer(s: String) -> i32 {
        let bs = s.as_bytes();
        let n = bs.len();
        let mut dp = vec![vec![0i32; n]; n];
        for l in 1..=n {
            for left in 0..=(n - l) {
                let right = left + l - 1;
                dp[left][right] = n as i32;

                let mut j = -1 as i32;
                for i in left..right {
                    if (j == -1) && (bs[i] != bs[right]) {
                        j = i as i32;
                    }

                    if j != -1 {
                        let lmin = dp[left][right];
                        let rmin = 1 + dp[j as usize][i] + dp[i + 1][right];
                        dp[left][right] = lmin.min(rmin);
                    }
                }

                if j == -1 {
                    dp[left][right] = 0;
                }
            }
        }

        return dp[0][n - 1] + 1;
    }
}

fn main() {
    let inputs = ["aaabbb", "aba"];

    for s in inputs {
        let result = Solution::strange_printer(s.to_string());
        println!("{result}");
    }
}

mod utils;

use utils::Solution;

impl Solution {
    pub fn longest_palindrome_subseq(s: String) -> i32 {
        let n = s.len();
        let mut dp = vec![0; n];
        let mut dp_prev = vec![0; n];

        let bytes = s.as_bytes();

        for i in (0..n).rev() {
            dp[i] = 1;
            for j in (i + 1)..n {
                if bytes[i] == bytes[j] {
                    dp[j] = dp_prev[j - 1] + 2;
                } else {
                    dp[j] = dp_prev[j].max(dp[j - 1]);
                }
            }

            dp_prev = dp.clone();
        }

        return dp[n - 1];
    }
}

fn main() {
    let inputs = ["bbbab", "cbbd"];

    for s in inputs {
        let result = Solution::longest_palindrome_subseq(s.to_string());
        println!("{result}");
    }
}

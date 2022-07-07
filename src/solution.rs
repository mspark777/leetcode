pub struct Solution {}

impl Solution {
    pub fn is_interleave(s1: String, s2: String, s3: String) -> bool {
        let b1 = s1.as_bytes();
        let b2 = s2.as_bytes();
        let b3 = s3.as_bytes();
        let b1_len = b1.len();
        let b2_len = b2.len();
        let b3_len = b3.len();
        if (b1_len + b2_len) != b3_len {
            return false;
        }

        let mut dp = vec![false; b2_len + 1];

        for i in 0..=b1_len {
            for j in 0..=b2_len {
                if (i == 0) && (j == 0) {
                    dp[j] = true;
                } else if i == 0 {
                    dp[j] = dp[j - 1] && b2[j - 1] == b3[i + j - 1];
                } else if j == 0 {
                    dp[j] = dp[j] && b1[i - 1] == b3[i + j - 1];
                } else {
                    dp[j] = (dp[j] && b1[i - 1] == b3[i + j - 1])
                        || (dp[j - 1] && b2[j - 1] == b3[i + j - 1]);
                }
            }
        }

        dp[b2_len]
    }
}

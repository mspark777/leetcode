pub struct Solution {}
impl Solution {
    pub fn k_inverse_pairs(n: i32, k: i32) -> i32 {
        let n = n as usize;
        let k = k as usize;
        let mut dp = vec![0; k + 1];

        const MODULO: i32 = 1000000007;

        for i in 1..=n {
            let mut temp = vec![0; k + 1];
            temp[0] = 1;

            for j in 1..=k {
                let mut v = dp[j] + MODULO;
                if j >= i {
                    v -= dp[j - i];
                }

                v %= MODULO;
                temp[j] = (temp[j - 1] + v) % MODULO;
            }
            dp = temp;
        }

        let mut result = dp[k] + MODULO;
        if k > 0 {
            result -= dp[k - 1];
        }

        return result % MODULO;
    }
}

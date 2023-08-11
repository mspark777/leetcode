mod utils;

use utils::Solution;

impl Solution {
    pub fn change(amount: i32, coins: Vec<i32>) -> i32 {
        let amount = amount as usize;
        let mut dp = vec![0; amount + 1];
        dp[0] = 1;

        for i in (0..coins.len()).rev() {
            let coin = coins[i] as usize;
            for j in coin..=amount {
                dp[j] += dp[j - coin];
            }
        }

        return dp[amount];
    }
}

fn main() {
    let inputs = [(5, vec![1, 2, 5]), (3, vec![2]), (10, vec![10])];

    for (amount, coins) in inputs {
        let result = Solution::change(amount, coins);
        println!("{result}");
    }
}

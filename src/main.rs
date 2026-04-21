struct Solution;

impl Solution {
    pub fn coin_change(coins: Vec<i32>, amount: i32) -> i32 {
        let mut dp = vec![amount + 1; (amount + 1) as usize];
        dp[0] = 0;

        for i in 1..=amount {
            for coin in coins.iter().copied() {
                if (i - coin) >= 0 {
                    dp[i as usize] = dp[i as usize].min(1 + dp[(i - coin) as usize]);
                }
            }
        }

        match dp[amount as usize] > amount {
            true => -1,
            _ => dp[amount as usize],
        }
    }
}

struct Input {
    coins: Vec<i32>,
    amount: i32,
}

fn main() {
    let inputs = [Input {
        coins: [1, 2, 5].to_vec(),
        amount: 11,
    }];

    for input in inputs {
        let result = Solution::coin_change(input.coins, input.amount);
        println!("{:?}", result);
    }
}

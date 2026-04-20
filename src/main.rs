struct Solution;

impl Solution {
    pub fn max_profit(prices: Vec<i32>) -> i32 {
        let mut dp = [[0; 2]; 3];

        for (day, price) in prices.into_iter().enumerate().rev() {
            for buy in 0..2usize {
                let first = dp[(day + 1) % 3][buy];
                let second = match buy {
                    1 => dp[(day + 1) % 3][0] - price,
                    _ => dp[(day + 2) % 3][1] + price,
                };
                dp[day % 3][buy] = first.max(second);
            }
        }

        dp[0][1]
    }
}

struct Input {
    prices: Vec<i32>,
}

fn main() {
    let inputs = [
        Input {
            prices: [1, 2, 3, 0, 2].to_vec(),
        },
        Input {
            prices: [1].to_vec(),
        },
    ];

    for input in inputs {
        let result = Solution::max_profit(input.prices);
        println!("{:?}", result);
    }
}

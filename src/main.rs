struct Solution;

impl Solution {
    pub fn max_profit(prices: Vec<i32>, fee: i32) -> i32 {
        let mut s0 = -prices[0];
        let mut s1 = 0;
        for price in prices.into_iter().skip(1) {
            let old_s0 = s0;
            s0 = s0.max(s1 - price);
            s1 = s1.max(old_s0 + price - fee);
        }
        s0.max(s1)
    }
}

struct Input {
    prices: Vec<i32>,
    fee: i32,
}

fn main() {
    let inputs = [Input {
        prices: [1, 3, 2, 8, 4, 9].to_vec(),
        fee: 2,
    }];

    for input in inputs.into_iter() {
        let result = Solution::max_profit(input.prices, input.fee);
        println!("{:?}", result);
    }
}

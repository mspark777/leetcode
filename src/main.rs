struct Solution {}

impl Solution {
    pub fn count_good_strings(low: i32, high: i32, zero: i32, one: i32) -> i32 {
        let low = low as usize;
        let high = high as usize;
        let zero = zero as usize;
        let one = one as usize;

        let mut dp = vec![0; high + 1];
        dp[0] = 1;
        let m = 1_000_000_007;

        for end in 1..=high {
            if end >= zero {
                dp[end] += dp[end - zero];
            }

            if end >= one {
                dp[end] += dp[end - one];
            }

            dp[end] %= m;
        }

        let mut result = 0;

        for i in low..=high {
            result += dp[i];
            result %= m;
        }

        return result;
    }
}

struct Input {
    low: i32,
    high: i32,
    zero: i32,
    one: i32,
}

fn main() {
    let inputs = vec![
        Input {
            low: 3,
            high: 3,
            zero: 1,
            one: 1,
        },
        Input {
            low: 3,
            high: 3,
            zero: 1,
            one: 2,
        },
    ];

    for input in inputs {
        let result = Solution::count_good_strings(input.low, input.high, input.zero, input.one);
        println!("{result:?}");
    }
}

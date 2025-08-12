struct Solution {}

impl Solution {
    pub fn number_of_ways(n: i32, x: i32) -> i32 {
        const MOD: i64 = 1_000_000_007;
        let n = n as usize;
        let x = x as u32;
        let mut dp = vec![0i64; n + 1];

        dp[0] = 1;
        for i in 1..=n {
            let val = i.pow(x);
            if val > n {
                break;
            }

            for j in (val..=n).rev() {
                dp[j] = (dp[j] + dp[j - val]) % MOD;
            }
        }

        dp[n] as i32
    }
}

struct Input {
    n: i32,
    x: i32,
}

fn main() {
    let inputs = [Input { n: 10, x: 2 }, Input { n: 4, x: 1 }];

    for input in inputs {
        let result = Solution::number_of_ways(input.n, input.x);
        println!("{:?}", result);
    }
}

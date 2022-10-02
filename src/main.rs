struct Solution {}
impl Solution {
    pub fn num_rolls_to_target(n: i32, k: i32, target: i32) -> i32 {
        let n = n as usize;
        let k = k as usize;
        let target = target as usize;

        const MOD: i32 = 1000000007;
        let mut dp = vec![0; target + 1];
        dp[0] = 1;

        for _ in 1..=n {
            for j in (0..=target).rev() {
                dp[j] = 0;

                for p in 1..=k {
                    if j >= p {
                        dp[j] = (dp[j] + dp[j - p]) % MOD;
                    } else {
                        break;
                    }
                }
            }
        }
        return dp[target];
    }
}

struct Input {
    n: i32,
    k: i32,
    target: i32,
}
fn main() {
    let inputs = vec![
        Input {
            n: 1,
            k: 6,
            target: 3,
        },
        Input {
            n: 2,
            k: 6,
            target: 7,
        },
        Input {
            n: 30,
            k: 30,
            target: 500,
        },
    ];

    for Input { n, k, target } in inputs.into_iter() {
        let result = Solution::num_rolls_to_target(n, k, target);
        println!("{result:?}");
    }
}

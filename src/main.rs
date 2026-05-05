struct Solution;

impl Solution {
    pub fn get_money_amount(n: i32) -> i32 {
        let n = n as usize;
        let mut dp = vec![vec![0i32; n + 1]; n + 1];

        for len in 2..=n {
            for start in 1..=(n + 1 - len) {
                let mut minres = i32::MAX;
                for piv in (start + (len - 1) / 2)..(start + len - 1) {
                    let res = (piv as i32) + dp[start][piv - 1].max(dp[piv + 1][start + len - 1]);
                    minres = minres.min(res as i32);
                }

                dp[start][start + len - 1] = minres;
            }
        }

        dp[1][n]
    }
}

struct Input {
    n: i32,
}

fn main() {
    let inputs = [Input { n: 10 }, Input { n: 1 }, Input { n: 2 }];

    for input in inputs.into_iter() {
        let result = Solution::get_money_amount(input.n);
        println!("{:?}", result);
    }
}

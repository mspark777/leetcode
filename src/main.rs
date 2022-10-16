struct Solution {}
impl Solution {
    pub fn min_difficulty(job_difficulty: Vec<i32>, d: i32) -> i32 {
        let d = d as usize;
        let days = job_difficulty.len();

        if days < d {
            return -1;
        }

        let mut dp = vec![0; days + 1];
        for i in (0..days).rev() {
            dp[i] = dp[i + 1].max(job_difficulty[i]);
        }

        for i in 2..=d {
            let remain = days - i;
            for j in 0..=remain {
                let mut maxd = 0;
                dp[j] = i32::max_value();

                for k in j..=remain {
                    maxd = maxd.max(job_difficulty[k]);
                    dp[j] = dp[j].min(maxd + dp[k + 1]);
                }
            }
        }

        return dp[0];
    }
}

struct Input {
    job_difficulty: Vec<i32>,
    d: i32,
}

fn main() {
    let inputs = [
        Input {
            job_difficulty: vec![6, 5, 4, 3, 2, 1],
            d: 2,
        },
        Input {
            job_difficulty: vec![9, 9, 9],
            d: 4,
        },
        Input {
            job_difficulty: vec![1, 1, 1],
            d: 3,
        },
    ];

    for Input { job_difficulty, d } in inputs {
        let result = Solution::min_difficulty(job_difficulty, d);
        println!("{result}");
    }
}

struct Solution;

impl Solution {
    pub fn largest_sum_of_averages(nums: Vec<i32>, k: i32) -> f64 {
        let n = nums.len();
        let mut p = vec![0; n + 1];
        for i in 0..n {
            p[i + 1] = p[i] + nums[i];
        }

        let mut dp = vec![0.0; n];
        for i in 0..n {
            dp[i] = ((p[n] - p[i]) as f64) / ((n - i) as f64);
        }

        for _ in 1..k {
            for i in 0..n {
                for j in (i + 1)..n {
                    dp[i] = dp[i].max(((p[j] - p[i]) as f64) / ((j - i) as f64) + dp[j])
                }
            }
        }

        dp[0]
    }
}

struct Input {
    nums: Vec<i32>,
    k: i32,
}

fn main() {
    let inputs = [Input {
        nums: [9, 1, 2, 3, 9].to_vec(),
        k: 3,
    }];

    for input in inputs {
        let result = Solution::largest_sum_of_averages(input.nums, input.k);
        println!("{:?}", result);
    }
}

mod utils;

use utils::Solution;

impl Solution {
    pub fn most_points(questions: Vec<Vec<i32>>) -> i64 {
        let qlen = questions.len();
        let last = qlen - 1;
        let mut dp = vec![0i64; qlen];
        dp[last] = questions[last][0] as i64;

        for i in (0..last).rev() {
            let question = &questions[i];
            let point = question[0] as i64;
            let power = question[1] as usize;

            dp[i] = point;
            if (i + power) < last {
                dp[i] += dp[i + power + 1];
            }

            dp[i] = dp[i].max(dp[i + 1]);
        }

        return dp[0];
    }
}

fn main() {
    let inputs = [
        vec![vec![3, 2], vec![4, 3], vec![4, 4], vec![2, 5]],
        vec![vec![1, 1], vec![2, 2], vec![3, 3], vec![4, 4], vec![5, 5]],
    ];

    for questions in inputs {
        let result = Solution::most_points(questions);
        println!("{result}");
    }
}

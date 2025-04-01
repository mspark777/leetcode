struct Solution {}

impl Solution {
    pub fn most_points(questions: Vec<Vec<i32>>) -> i64 {
        let n = questions.len();
        let mut dp = vec![0i64; n];
        dp[n - 1] = questions[n - 1][0] as i64;

        for i in (0..(n - 1)).rev() {
            let question = &questions[i];
            let points = question[0] as i64;
            let brainpower = question[1] as usize;
            let next_idx = i + brainpower + 1;
            let mut solve_points = points;
            if next_idx < n {
                solve_points += dp[next_idx];
            }

            let skip_points = dp[i + 1];
            dp[i] = solve_points.max(skip_points);
        }

        return dp[0];
    }
}

struct Input {
    questions: Vec<Vec<i32>>,
}

fn main() {
    let inputs = vec![
        Input {
            questions: vec![vec![3, 2], vec![4, 3], vec![4, 4], vec![2, 5]],
        },
        Input {
            questions: vec![vec![1, 1], vec![2, 2], vec![3, 3], vec![4, 4], vec![5, 5]],
        },
    ];

    for input in inputs {
        let result = Solution::most_points(input.questions);
        println!("{result:?}");
    }
}

struct Solution {}
impl Solution {
    pub fn min_falling_path_sum(matrix: Vec<Vec<i32>>) -> i32 {
        let row_count = matrix.len();
        let col_count = matrix[0].len();
        let last_row_idx = row_count - 1;
        let last_col_idx = col_count - 1;

        let mut dp = vec![vec![0; col_count]; row_count];
        for i in 0..col_count {
            dp[last_row_idx][i] = matrix[last_row_idx][i];
        }

        for i in (0..(row_count - 1)).rev() {
            for j in 0..col_count {
                let next = i + 1;
                let mut min = i32::max_value();

                if j < last_col_idx {
                    min = min.min(dp[next][j + 1]);
                }

                if j > 0 {
                    min = min.min(dp[next][j - 1]);
                }

                min = min.min(dp[next][j]);
                dp[i][j] = matrix[i][j] + min;
            }
        }

        let mut min = i32::max_value();
        for i in 0..col_count {
            min = min.min(dp[0][i]);
        }

        return min;
    }
}

fn main() {
    let inputs = [
        vec![vec![2, 1, 3], vec![6, 5, 4], vec![7, 8, 9]],
        vec![vec![-19, 57], vec![-40, -5]],
    ];

    for matrix in inputs {
        let result = Solution::min_falling_path_sum(matrix);
        println!("{result}");
    }
}

#[allow(dead_code)]
pub struct NumMatrix {
    matrix: Vec<i32>,
    col_len: usize,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
#[allow(dead_code)]
impl NumMatrix {
    fn new(matrix: Vec<Vec<i32>>) -> Self {
        let row_len = matrix.len();
        let col_len = matrix[0].len();
        let dp_row_len = row_len + 1;
        let dp_col_len = col_len + 1;
        let mut dp = vec![0; dp_row_len * dp_col_len];

        for i in 0..row_len {
            for j in 0..col_len {
                let v0 = dp[Self::get_index(i + 1, j, dp_col_len)];
                let v1 = dp[Self::get_index(i, j + 1, dp_col_len)];
                let v2 = matrix[i][j];
                let v3 = dp[Self::get_index(i, j, dp_col_len)];
                let v = v0 + v1 + v2 - v3;
                dp[Self::get_index(i + 1, j + 1, dp_col_len)] = v;
            }
        }

        NumMatrix {
            matrix: dp,
            col_len: dp_col_len,
        }
    }

    fn get_index(i: usize, j: usize, col: usize) -> usize {
        i * col + j
    }

    fn sum_region(&self, row1: i32, col1: i32, row2: i32, col2: i32) -> i32 {
        let dp = &self.matrix;
        let col_len = self.col_len;
        let get_dp = |i: i32, j: i32| dp[Self::get_index(i as usize, j as usize, col_len)];
        let v0 = get_dp(row2 + 1, col2 + 1);
        let v1 = get_dp(row1, col2 + 1);
        let v2 = get_dp(row2 + 1, col1);
        let v3 = get_dp(row1, col1);
        let v = v0 - v1 - v2 + v3;
        v
    }
}

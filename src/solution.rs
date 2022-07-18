pub struct Solution {}
impl Solution {
    pub fn num_submatrix_sum_target(matrix: Vec<Vec<i32>>, target: i32) -> i32 {
        use std::collections::HashMap;

        let mut matrix = matrix;
        let mut result = 0;
        let m = matrix.len();
        let n = matrix[0].len();

        for i in 0..m {
            for j in 1..n {
                matrix[i][j] += matrix[i][j - 1];
            }
        }

        let mut counter = HashMap::<i32, i32>::with_capacity(n);
        for i in 0..n {
            for j in i..n {
                counter.clear();
                counter.insert(0, 1);
                let mut cur = 0;

                for k in 0..m {
                    cur += matrix[k][j];
                    if i > 0 {
                        cur -= matrix[k][i - 1];
                    }

                    result += counter.get(&(cur - target)).unwrap_or(&0);
                    if let Some(c) = counter.get_mut(&cur) {
                        *c += 1;
                    } else {
                        counter.insert(cur, 1);
                    }
                }
            }
        }

        result
    }
}

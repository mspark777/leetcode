struct Solution {}

impl Solution {
    pub fn max_matrix_sum(matrix: Vec<Vec<i32>>) -> i64 {
        let mut result = 0i64;
        let mut min_abs_val = i64::MAX;
        let mut negative_count = 0i64;

        for row in matrix.iter() {
            for &val in row.iter() {
                let val_abs = val.abs() as i64;
                result += val_abs;
                if val < 0 {
                    negative_count += 1;
                }

                min_abs_val = min_abs_val.min(val_abs);
            }
        }

        if negative_count & 1 == 1 {
            result -= 2 * min_abs_val;
        }

        return result;
    }
}

struct Input {
    matrix: Vec<Vec<i32>>,
}

fn main() {
    let inputs = vec![
        Input {
            matrix: vec![vec![1, -1], vec![-1, 1]],
        },
        Input {
            matrix: vec![vec![1, 2, 3], vec![-1, -2, -3], vec![1, 2, 3]],
        },
    ];

    for input in inputs {
        let result = Solution::max_matrix_sum(input.matrix);
        println!("{result}");
    }
}

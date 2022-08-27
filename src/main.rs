struct Solution {}
impl Solution {
    pub fn max_sum_submatrix(matrix: Vec<Vec<i32>>, k: i32) -> i32 {
        let row_count = matrix.len();
        let col_count = matrix[0].len();
        let mut max_sum = i32::min_value();

        for i0 in 0..col_count {
            let mut sums = vec![0; row_count];
            for i1 in i0..col_count {
                for i2 in 0..row_count {
                    sums[i2] += matrix[i2][i1];
                }

                for i2 in 0..row_count {
                    let mut sum = 0;
                    for i3 in i2..row_count {
                        sum += sums[i3];
                        if (sum > max_sum) && (sum <= k) {
                            max_sum = sum;
                        }
                    }
                }
            }
        }

        max_sum
    }
}

struct Input {
    matrix: Vec<Vec<i32>>,
    k: i32,
}

fn main() {
    let inputs: Vec<Input> = vec![
        Input {
            matrix: vec![vec![1, 0, 1], vec![0, -2, 3]],
            k: 2,
        },
        Input {
            matrix: vec![vec![2, 2, -1]],
            k: 3,
        },
    ];

    for input in inputs.iter() {
        let matrix = input.matrix.clone();
        let k = input.k;
        let result = Solution::max_sum_submatrix(matrix, k);
        println!("{result}");
    }
}

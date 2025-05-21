struct Solution {}

impl Solution {
    pub fn set_zeroes(matrix: &mut Vec<Vec<i32>>) {
        let n = matrix.len();
        let m = matrix[0].len();
        let mut fr = false;
        let mut fc = false;

        for r in 0..n {
            for c in 0..m {
                if matrix[r][c] != 0 {
                    continue;
                }

                fr = if fr { true } else { r == 0 };
                fc = if fc { true } else { c == 0 };
                matrix[0][c] = 0;
                matrix[r][0] = 0;
            }
        }

        for r in 1..n {
            for c in 1..m {
                if matrix[r][0] == 0 {
                    matrix[r][c] = 0;
                } else if matrix[0][c] == 0 {
                    matrix[r][c] = 0;
                }
            }
        }

        if fr {
            for c in 0..m {
                matrix[0][c] = 0;
            }
        }

        if fc {
            for r in 0..n {
                matrix[r][0] = 0;
            }
        }
    }
}

struct Input {
    matrix: Vec<Vec<i32>>,
}

fn main() {
    let inputs = vec![
        Input {
            matrix: vec![vec![1, 1, 1], vec![1, 0, 1], vec![1, 1, 1]],
        },
        Input {
            matrix: vec![vec![0, 1, 2, 0], vec![3, 4, 5, 2], vec![1, 3, 1, 5]],
        },
    ];

    for mut input in inputs {
        Solution::set_zeroes(&mut input.matrix);
        println!("{:?}", input.matrix);
    }
}

struct Solution {}
impl Solution {
    pub fn rotate(matrix: &mut Vec<Vec<i32>>) {
        Self::transpose(matrix);
        Self::reverse(matrix);
    }

    fn transpose(matrix: &mut Vec<Vec<i32>>) {
        for i in 0..matrix.len() {
            for j in (i + 1)..matrix.len() {
                let temp = matrix[i][j];
                matrix[i][j] = matrix[j][i];
                matrix[j][i] = temp;
            }
        }
    }

    fn reverse(matrix: &mut Vec<Vec<i32>>) {
        for i in 0..matrix.len() {
            let mut j = 0usize;
            let mut k = matrix.len() - 1;
            while j < k {
                let temp = matrix[i][j];
                matrix[i][j] = matrix[i][k];
                matrix[i][k] = temp;

                j += 1;
                k -= 1;
            }
        }
    }
}

struct Input {
    matrix: Vec<Vec<i32>>,
}

fn main() {
    let mut inputs: Vec<Input> = vec![
        Input {
            matrix: vec![vec![1, 2, 3], vec![4, 5, 6], vec![7, 8, 9]],
        },
        Input {
            matrix: vec![
                vec![5, 1, 9, 11],
                vec![2, 4, 8, 10],
                vec![13, 3, 6, 7],
                vec![15, 14, 12, 16],
            ],
        },
    ];

    for input in inputs.iter_mut() {
        let matrix = &mut input.matrix;
        Solution::rotate(matrix);
        println!("{matrix:?}");
    }
}

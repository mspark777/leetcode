struct Solution;

impl Solution {
    pub fn modified_matrix(matrix: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        let mut result = matrix.clone();
        let n = matrix[0].len();

        for c in 0..n {
            let mut max_value = -1;
            for row in matrix.iter() {
                max_value = max_value.max(row[c]);
            }

            for row in result.iter_mut() {
                if row[c] == -1 {
                    row[c] = max_value;
                }
            }
        }

        result
    }
}

struct Input {
    matrix: Vec<Vec<i32>>,
}

fn main() {
    let inputs = [
        Input {
            matrix: [[1, 2, -1], [4, -1, 6], [7, 8, 9]]
                .map(|v| v.to_vec())
                .to_vec(),
        },
        Input {
            matrix: [[3, -1], [5, 2]].map(|v| v.to_vec()).to_vec(),
        },
    ];

    for input in inputs {
        let result = Solution::modified_matrix(input.matrix);
        println!("{:?}", result);
    }
}

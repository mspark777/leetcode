struct Solution {}
impl Solution {
    pub fn is_toeplitz_matrix(matrix: Vec<Vec<i32>>) -> bool {
        for r in 1..matrix.len() {
            for c in 1..matrix[r].len() {
                if matrix[r - 1][c - 1] != matrix[r][c] {
                    return false;
                }
            }
        }

        return true;
    }
}

fn main() {
    let inputs = [
        vec![vec![1, 2, 3, 4], vec![5, 1, 2, 3], vec![9, 5, 1, 2]],
        vec![vec![1, 2], vec![2, 2]],
    ];

    for matrix in inputs {
        let result = Solution::is_toeplitz_matrix(matrix);
        println!("{result}");
    }
}

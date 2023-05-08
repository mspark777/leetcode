mod utils;

use utils::Solution;

impl Solution {
    pub fn diagonal_sum(mat: Vec<Vec<i32>>) -> i32 {
        let mut result = 0;
        for left in 0..mat.len() {
            let right = mat.len() - (left + 1);
            result += mat[left][left] + mat[right][left];
        }

        let m = mat.len() / 2;
        result -= mat[m][m] * ((mat.len() & 1) as i32);

        return result;
    }
}

fn main() {
    let inputs = [
        vec![vec![1, 2, 3], vec![4, 5, 6], vec![7, 8, 9]],
        vec![
            vec![1, 1, 1, 1],
            vec![1, 1, 1, 1],
            vec![1, 1, 1, 1],
            vec![1, 1, 1, 1],
        ],
        vec![vec![5]],
    ];

    for mat in inputs {
        let result = Solution::diagonal_sum(mat);
        println!("{result}");
    }
}

pub struct Solution {}
impl Solution {
    pub fn search_matrix(matrix: Vec<Vec<i32>>, target: i32) -> bool {
        let mut row = matrix.len() - 1;
        let mut col = 0usize;
        let countcol = matrix[0].len();

        while col < countcol {
            let n = matrix[row][col];
            if target > n {
                col += 1;
            } else if target < n {
                if row > 0 {
                    row -= 1;
                } else {
                    return false;
                }
            } else {
                return true;
            }
        }

        false
    }
}

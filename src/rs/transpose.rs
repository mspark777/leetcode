#[allow(dead_code)]
pub struct Transpose {}

#[allow(dead_code)]
impl Transpose {
    pub fn transpose(matrix: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        let row_len = matrix.len();
        let col_len = matrix[0].len();
        let mut transposed: Vec<Vec<i32>> = Vec::with_capacity(col_len);
        for i in 0..col_len {
            let mut row: Vec<i32> = Vec::with_capacity(row_len);
            for j in 0..row_len {
                let col = matrix[j][i];
                row.push(col);
            }
            transposed.push(row);
        }

        transposed
    }

    pub fn transpose1(matrix: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        let row_len = matrix.len();
        let col_len = matrix[0].len();
        let mut transposed = vec![vec![0; row_len]; col_len];
        for i in 0..col_len {
            for j in 0..row_len {
                transposed[i][j] = matrix[j][i];
            }
        }

        transposed
    }
}

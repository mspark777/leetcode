#[allow(dead_code)]
pub struct MinimumTotal {}

#[allow(dead_code)]
impl MinimumTotal {
    pub fn minimum_total(triangle: Vec<Vec<i32>>) -> i32 {
        let len = triangle.len();
        if len < 2 {
            return triangle[0][0];
        }

        let mut triangle = triangle;
        for i in (0..=len - 2).rev() {
            for j in 0..triangle[i].len() {
                let n0 = triangle[i + 1][j];
                let n1 = triangle[i + 1][j + 1];
                triangle[i][j] += n0.min(n1);
            }
        }
        triangle[0][0]
    }
}

#[allow(dead_code)]
pub struct MinimumTotal1 {}

#[allow(dead_code)]
impl MinimumTotal1 {
    pub fn minimum_total(triangle: Vec<Vec<i32>>) -> i32 {
        let len = triangle.len();
        let mut triangle = triangle;

        for i in (0..len).rev().skip(1) {
            for j in 0..triangle[i].len() {
                let n0 = triangle[i + 1][j];
                let n1 = triangle[i + 1][j + 1];
                triangle[i][j] += n0.min(n1);
            }
        }
        triangle[0][0]
    }
}

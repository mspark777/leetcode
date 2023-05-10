mod utils;

use utils::Solution;

impl Solution {
    pub fn generate_matrix(n: i32) -> Vec<Vec<i32>> {
        let mut result = vec![vec![0 as i32; n as usize]; n as usize];
        let dir = [(0, 1), (1, 0), (0, -1), (-1, 0)];
        let mut d = 0;
        let mut row = 0;
        let mut col = 0;

        for cnt in 1..=(n * n) {
            result[row as usize][col as usize] = cnt as i32;
            let r = Self::floor_mod(row + dir[d as usize].0, n) as usize;
            let c = Self::floor_mod(col + dir[d as usize].1, n) as usize;

            if result[r][c] != 0 {
                d = (d + 1) % 4;
            }

            row += dir[d].0;
            col += dir[d].1;
        }

        return result;
    }

    fn floor_mod(x: i32, y: i32) -> i32 {
        return ((x % y) + y) % y;
    }
}

fn main() {
    let inputs = [3, 1];

    for n in inputs {
        let result = Solution::generate_matrix(n);
        println!("{result:?}");
    }
}

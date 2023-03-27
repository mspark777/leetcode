mod utils;

use utils::Solution;

impl Solution {
    pub fn min_path_sum(grid: Vec<Vec<i32>>) -> i32 {
        let mut grid = grid;
        let row = grid.len();
        let col = grid[0].len();
        let last_row = row - 1;
        let last_col = col - 1;
        const MAX: i32 = i32::max_value();

        for r in (0..=last_row).rev() {
            for c in (0..=last_col).rev() {
                if (r == last_row) && (c == last_col) {
                    continue;
                }

                let right_min = if c < last_col { grid[r][c + 1] } else { MAX };
                let down_min = if r < last_row { grid[r + 1][c] } else { MAX };
                grid[r][c] += right_min.min(down_min);
            }
        }

        return grid[0][0];
    }
}

fn main() {
    let inputs = [
        vec![vec![1, 3, 1], vec![1, 5, 1], vec![4, 2, 1]],
        vec![vec![1, 2, 3], vec![4, 5, 6]],
    ];

    for grid in inputs {
        let result = Solution::min_path_sum(grid);
        println!("{result}")
    }
}

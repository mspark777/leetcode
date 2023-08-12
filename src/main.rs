mod utils;

use utils::Solution;

impl Solution {
    pub fn unique_paths_with_obstacles(obstacle_grid: Vec<Vec<i32>>) -> i32 {
        const OBSTACLE: i32 = 1;
        let row_count = obstacle_grid.len();
        let col_count = obstacle_grid[0].len();
        let mut count_grid = vec![vec![0; col_count]; row_count];

        for r in 0..row_count {
            for c in 0..col_count {
                if obstacle_grid[r][c] == OBSTACLE {
                    continue;
                }

                if (r + c) == 0 {
                    count_grid[r][c] = 1;
                } else if r == 0 {
                    count_grid[r][c] = count_grid[r][c - 1]
                } else if c == 0 {
                    count_grid[r][c] = count_grid[r - 1][c]
                } else {
                    count_grid[r][c] = count_grid[r - 1][c] + count_grid[r][c - 1];
                }
            }
        }

        return count_grid[row_count - 1][col_count - 1];
    }
}

fn main() {
    let inputs = [
        vec![vec![0, 0, 0], vec![0, 1, 0], vec![0, 0, 0]],
        vec![vec![0, 1], vec![0, 0]],
    ];

    for grid in inputs {
        let result = Solution::unique_paths_with_obstacles(grid);
        println!("{result}");
    }
}

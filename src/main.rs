struct Solution {}

impl Solution {
    pub fn projection_area(grid: Vec<Vec<i32>>) -> i32 {
        let n = grid.len();
        let mut result = 0;

        for r in 0..n {
            let mut best_row = 0;
            let mut best_col = 0;
            for c in 0..n {
                if grid[r][c] > 0 {
                    result += 1;
                }

                best_row = best_row.max(grid[r][c]);
                best_col = best_col.max(grid[c][r]);
            }

            result += best_row + best_col;
        }

        return result;
    }
}

struct Input {
    grid: Vec<Vec<i32>>,
}

fn main() {
    let inputs = vec![
        Input {
            grid: vec![vec![1, 2], vec![3, 4]],
        },
        Input {
            grid: vec![vec![2]],
        },
        Input {
            grid: vec![vec![1, 0], vec![0, 2]],
        },
    ];

    for input in inputs {
        let result = Solution::projection_area(input.grid);
        println!("{result:?}");
    }
}

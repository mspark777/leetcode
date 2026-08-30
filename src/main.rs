struct Solution;

impl Solution {
    pub fn num_enclaves(mut grid: Vec<Vec<i32>>) -> i32 {
        let m = grid.len();
        let n = grid[0].len();

        for i in 0..m {
            if grid[i][0] == 1 {
                Self::dfs(&mut grid, i, 0);
            }
            if grid[i][n - 1] == 1 {
                Self::dfs(&mut grid, i, n - 1);
            }
        }
        for j in 0..n {
            if grid[0][j] == 1 {
                Self::dfs(&mut grid, 0, j);
            }
            if grid[m - 1][j] == 1 {
                Self::dfs(&mut grid, m - 1, j);
            }
        }

        grid.iter().map(|row| row.iter().sum::<i32>()).sum()
    }

    fn dfs(grid: &mut Vec<Vec<i32>>, i: usize, j: usize) {
        if i >= grid.len() || j >= grid[0].len() || grid[i][j] == 0 {
            return;
        }
        grid[i][j] = 0;
        if i > 0 {
            Self::dfs(grid, i - 1, j);
        }
        if i + 1 < grid.len() {
            Self::dfs(grid, i + 1, j);
        }
        if j > 0 {
            Self::dfs(grid, i, j - 1);
        }
        if j + 1 < grid[0].len() {
            Self::dfs(grid, i, j + 1);
        }
    }
}

struct Input {
    n: i32,
}

fn main() {
    let inputs = [Input { n: 3 }];

    for input in inputs {
        let result = Solution::base_neg2(input.n);
        println!("{:?}", result);
    }
}

struct Solution;

impl Solution {
    fn is_valid(i: i32, j: i32, n: i32, m: i32) -> bool {
        i >= 0 && j >= 0 && i < n && j < m
    }

    pub fn color_border(grid: Vec<Vec<i32>>, row: i32, col: i32, color: i32) -> Vec<Vec<i32>> {
        let row = row as usize;
        let col = col as usize;
        let n = grid.len();
        let m = grid[0].len();
        let directions = [(0, 1), (1, 0), (0, -1), (-1, 0)];

        let mut grid = grid;
        let mut queue = std::collections::VecDeque::new();
        let mut is_visited = vec![vec![false; m]; n];
        let mut border_cells = vec![];

        let original_color = grid[row][col];
        queue.push_back((row, col));
        is_visited[row][col] = true;

        while let Some((i, j)) = queue.pop_front() {
            let mut is_border = false;
            for &(di, dj) in &directions {
                let ni = i as i32 + di;
                let nj = j as i32 + dj;

                if !Self::is_valid(ni, nj, n as i32, m as i32) {
                    is_border = true;
                    continue;
                }

                let ni = ni as usize;
                let nj = nj as usize;

                if grid[ni][nj] != original_color {
                    is_border = true;
                } else if !is_visited[ni][nj] {
                    is_visited[ni][nj] = true;
                    queue.push_back((ni, nj));
                }
            }

            if is_border {
                border_cells.push((i, j));
            }
        }

        for (i, j) in border_cells {
            grid[i][j] = color;
        }

        grid
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

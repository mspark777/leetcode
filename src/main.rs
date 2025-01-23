struct Solution {}

impl Solution {
    pub fn count_servers(grid: Vec<Vec<i32>>) -> i32 {
        let mut result = 0;
        let mut row_counts = vec![0; grid[0].len()];
        let mut last_server_in_col = vec![grid[0].len(); grid.len()];

        for (r, row) in grid.iter().enumerate() {
            let mut server_count_in_row = 0;
            for (c, cell) in row.iter().cloned().enumerate() {
                if cell == 1 {
                    server_count_in_row += 1;
                    row_counts[c] += 1;
                    last_server_in_col[r] = c;
                }
            }

            if server_count_in_row != 1 {
                result += server_count_in_row;
                last_server_in_col[r] = grid[0].len();
            }
        }

        for r in 0..grid.len() {
            if (last_server_in_col[r] != grid[0].len()) && (row_counts[last_server_in_col[r]] > 1) {
                result += 1;
            }
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
            grid: vec![vec![1, 0], vec![0, 1]],
        },
        Input {
            grid: vec![vec![1, 0], vec![1, 1]],
        },
        Input {
            grid: vec![
                vec![1, 1, 0, 0],
                vec![0, 0, 1, 0],
                vec![0, 0, 1, 0],
                vec![0, 0, 0, 1],
            ],
        },
    ];

    for input in inputs {
        let result = Solution::count_servers(input.grid);
        println!("{result:?}");
    }
}

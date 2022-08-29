const LAND: char = '1';
const WATER: char = '0';

struct Solution {}
impl Solution {
    pub fn num_islands(grid: Vec<Vec<char>>) -> i32 {
        let mut grid = grid;
        let row_count = grid.len();
        let col_count = grid[0].len();

        let mut result = 0;
        for r in 0..row_count {
            for c in 0..col_count {
                if grid[r][c] == LAND {
                    result += 1;
                    Self::clear_land(&mut grid, r, c, row_count, col_count);
                }
            }
        }

        result
    }

    fn clear_land(
        grid: &mut Vec<Vec<char>>,
        row: usize,
        col: usize,
        row_count: usize,
        col_count: usize,
    ) {
        let mut stack = vec![(row as i32, col as i32)];

        while let Some((r, c)) = stack.pop() {
            if (r < 0) || (c < 0) {
                continue;
            }

            let ridx = r as usize;
            let cidx = c as usize;
            if (ridx >= row_count) || (cidx >= col_count) {
                continue;
            } else if grid[ridx][cidx] == WATER {
                continue;
            }

            grid[ridx][cidx] = WATER;
            stack.push((r - 1, c));
            stack.push((r + 1, c));
            stack.push((r, c - 1));
            stack.push((r, c + 1));
        }
    }
}

struct Input {
    grid: Vec<Vec<char>>,
}

fn main() {
    let inputs: Vec<Input> = vec![
        Input {
            grid: vec![
                vec!['1', '1', '1', '1', '0'],
                vec!['1', '1', '0', '1', '0'],
                vec!['1', '1', '0', '0', '0'],
                vec!['0', '0', '0', '0', '0'],
            ],
        },
        Input {
            grid: vec![
                vec!['1', '1', '0', '0', '0'],
                vec!['1', '1', '0', '0', '0'],
                vec!['0', '0', '1', '0', '0'],
                vec!['0', '0', '0', '1', '1'],
            ],
        },
    ];

    for input in inputs.iter() {
        let grid = input.grid.clone();
        let result = Solution::num_islands(grid);
        println!("{result:?}");
    }
}

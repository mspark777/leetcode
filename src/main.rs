use std::cmp::Reverse;
use std::collections::BinaryHeap;

struct Solution {}

impl Solution {
    pub fn minimum_time(grid: Vec<Vec<i32>>) -> i32 {
        if grid[0][1] > 1 && grid[1][0] > 1 {
            return -1;
        }

        let rows = grid.len();
        let cols = grid[0].len();
        let last_row = rows - 1;
        let last_col = cols - 1;

        let directions = [(1, 0), (-1, 0), (0, 1), (0, -1)];
        let mut visiteds = vec![vec![false; cols]; rows];

        let mut queue = BinaryHeap::<Reverse<(i32, usize, usize)>>::new();
        queue.push(Reverse((grid[0][0], 0, 0)));

        while let Some(Reverse((time, row, col))) = queue.pop() {
            if row == last_row && col == last_col {
                return time;
            }

            if visiteds[row][col] {
                continue;
            }

            visiteds[row][col] = true;
            for &(dr, dc) in directions.iter() {
                let may_next_row = Self::get_next(row, dr, last_row);
                let may_next_col = Self::get_next(col, dc, last_col);

                if Self::is_invalid(&visiteds, may_next_row, may_next_col) {
                    continue;
                }

                let next_row = may_next_row.unwrap();
                let next_col = may_next_col.unwrap();
                let wati_time = if ((grid[next_row][next_col] - time) & 1) == 0 {
                    1
                } else {
                    0
                };

                let next_time = (time + 1).max(grid[next_row][next_col] + wati_time);
                queue.push(Reverse((next_time, next_row, next_col)));
            }
        }

        return -1;
    }

    fn get_next(idx: usize, dir: i32, last: usize) -> Option<usize> {
        let i = idx as i32;
        let next = i + dir;
        if next < 0 {
            return None;
        }

        return if (next as usize) > last {
            None
        } else {
            Some(next as usize)
        };
    }

    fn is_invalid(
        visited: &Vec<Vec<bool>>,
        may_row: Option<usize>,
        may_col: Option<usize>,
    ) -> bool {
        if may_row.is_none() || may_col.is_none() {
            return true;
        }

        let row = may_row.unwrap();
        let col = may_col.unwrap();

        return visited[row][col];
    }
}

struct Input {
    grid: Vec<Vec<i32>>,
}

fn main() {
    let inputs = vec![
        Input {
            grid: vec![vec![0, 1, 3, 2], vec![5, 1, 2, 5], vec![4, 3, 8, 6]],
        },
        Input {
            grid: vec![vec![0, 2, 4], vec![3, 2, 1], vec![1, 0, 4]],
        },
    ];

    for input in inputs {
        let result = Solution::minimum_time(input.grid);
        println!("{result:?}");
    }
}

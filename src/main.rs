use std::collections::VecDeque;

struct Solution {}

impl Solution {
    pub fn minimum_obstacles(grid: Vec<Vec<i32>>) -> i32 {
        let directions = [[0, 1], [0, -1], [1, 0], [-1, 0]];
        let m = grid.len();
        let n = grid[0].len();

        let mut min_obstacles = vec![vec![i32::MAX; n]; m];
        min_obstacles[0][0] = 0;

        let mut deque = VecDeque::<(i32, usize, usize)>::new();
        deque.push_back((0, 0, 0));

        while let Some(current) = deque.pop_front() {
            let obstacle = current.0;
            let row = current.1;
            let col = current.2;

            for direction in directions.iter() {
                let maybe_next_row = Self::get_next(row, direction[0], m);
                let maybe_next_col = Self::get_next(col, direction[1], n);
                if let (Some(next_row), Some(next_col)) = (maybe_next_row, maybe_next_col) {
                    let min_obstacle = min_obstacles[next_row][next_col];
                    if min_obstacle != i32::MAX {
                        continue;
                    }

                    if grid[next_row][next_col] == 1 {
                        min_obstacles[next_row][next_col] = obstacle + 1;
                        deque.push_back((obstacle + 1, next_row, next_col));
                    } else {
                        min_obstacles[next_row][next_col] = obstacle;
                        deque.push_front((obstacle, next_row, next_col));
                    }
                } else {
                    continue;
                }
            }
        }

        return min_obstacles[m - 1][n - 1];
    }

    fn get_next(current: usize, direction: i32, max: usize) -> Option<usize> {
        let next = (current as i32) + direction;
        let m = max as i32;

        return if next < 0 {
            None
        } else if next >= m {
            None
        } else {
            Some(next as usize)
        };
    }
}

struct Input {
    grid: Vec<Vec<i32>>,
}

fn main() {
    let inputs = vec![
        Input {
            grid: vec![vec![0, 1, 1], vec![1, 1, 0], vec![1, 1, 0]],
        },
        Input {
            grid: vec![
                vec![0, 1, 0, 0, 0],
                vec![0, 1, 0, 1, 0],
                vec![0, 0, 0, 1, 0],
            ],
        },
    ];

    for input in inputs {
        let result = Solution::minimum_obstacles(input.grid);
        println!("{result:?}");
    }
}

struct Solution {}

impl Solution {
    pub fn min_cost(grid: Vec<Vec<i32>>) -> i32 {
        let n = grid.len();
        let m = grid[0].len();

        let mut min_cost = vec![vec![i32::MAX; m]; n];
        min_cost[0][0] = 0;

        let mut queue = std::collections::VecDeque::<(usize, usize)>::with_capacity(n * m);
        queue.push_back((0, 0));

        let dirs = [(0, 1), (0, -1), (1, 0), (-1, 0)];

        while let Some((row, col)) = queue.pop_front() {
            for (dir, (dx, dy)) in dirs.iter().cloned().enumerate() {
                let maybe_new_row = Self::next_idx(row, dx, n);
                let maybe_new_col = Self::next_idx(col, dy, m);
                if let Some((new_row, new_col)) = maybe_new_row.zip(maybe_new_col) {
                    let mut cost = 0;
                    if grid[row][col] != ((dir + 1) as i32) {
                        cost = 1;
                    }

                    let new_cost = min_cost[row][col] + cost;
                    if new_cost < min_cost[new_row][new_col] {
                        min_cost[new_row][new_col] = new_cost;

                        if cost == 1 {
                            queue.push_back((new_row, new_col));
                        } else {
                            queue.push_front((new_row, new_col));
                        }
                    }
                }
            }
        }

        return min_cost.last().unwrap().last().unwrap().clone();
    }

    fn next_idx(curr: usize, d: i32, end: usize) -> Option<usize> {
        if d >= 0 {
            let next = curr + (d as usize);
            return if next < end { Some(next) } else { None };
        }

        let d = (-d) as usize;
        return if curr < d { None } else { Some(curr - d) };
    }
}

struct Input {
    grid: Vec<Vec<i32>>,
}

fn main() {
    let inputs = vec![
        Input {
            grid: vec![
                vec![1, 1, 1, 1],
                vec![2, 2, 2, 2],
                vec![1, 1, 1, 1],
                vec![2, 2, 2, 2],
            ],
        },
        Input {
            grid: vec![vec![1, 1, 3], vec![3, 2, 2], vec![1, 1, 4]],
        },
        Input {
            grid: vec![vec![1, 2], vec![4, 3]],
        },
    ];

    for input in inputs {
        let result = Solution::min_cost(input.grid);
        println!("{result:?}");
    }
}

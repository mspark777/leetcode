struct Solution {}

#[derive(PartialEq, Copy, Clone)]
enum Cell {
    GUARDED,
    UNGUARDED,
    WALL,
    GUARD,
}

impl Solution {
    pub fn count_unguarded(m: i32, n: i32, guards: Vec<Vec<i32>>, walls: Vec<Vec<i32>>) -> i32 {
        let g: Vec<Vec<usize>> = guards
            .iter()
            .map(|r| -> Vec<usize> { r.iter().map(|&c| c as usize).collect() })
            .collect();

        let w: Vec<Vec<usize>> = walls
            .iter()
            .map(|r| -> Vec<usize> { r.iter().map(|&c| c as usize).collect() })
            .collect();

        return Self::solve(m as usize, n as usize, g, w);
    }

    fn solve(m: usize, n: usize, guards: Vec<Vec<usize>>, walls: Vec<Vec<usize>>) -> i32 {
        let mut grid = vec![vec![Cell::UNGUARDED; n]; m];

        for guard in guards.iter() {
            let row = guard[0];
            let col = guard[1];
            grid[row][col] = Cell::GUARD;
        }

        for wall in walls.iter() {
            let row = wall[0];
            let col = wall[1];
            grid[row][col] = Cell::WALL;
        }

        for guard in guards.iter() {
            let row = guard[0];
            let col = guard[1];
            Self::mark_unguarded(row, col, &mut grid, m, n);
        }

        let mut result = 0;
        for row in grid.iter() {
            for &cell in row.iter() {
                if cell == Cell::UNGUARDED {
                    result += 1;
                }
            }
        }
        return result;
    }

    fn mark_unguarded(row: usize, col: usize, grid: &mut Vec<Vec<Cell>>, m: usize, n: usize) {
        for r in (0..row).rev() {
            let cell = grid[r][col];
            if cell == Cell::WALL {
                break;
            } else if cell == Cell::GUARD {
                break;
            } else {
                grid[r][col] = Cell::GUARDED;
            }
        }

        for r in (row + 1)..m {
            let cell = grid[r][col];
            if cell == Cell::WALL {
                break;
            } else if cell == Cell::GUARD {
                break;
            } else {
                grid[r][col] = Cell::GUARDED;
            }
        }

        for c in (0..col).rev() {
            let cell = grid[row][c];
            if cell == Cell::WALL {
                break;
            } else if cell == Cell::GUARD {
                break;
            } else {
                grid[row][c] = Cell::GUARDED;
            }
        }

        for c in (col + 1)..n {
            let cell = grid[row][c];
            if cell == Cell::WALL {
                break;
            } else if cell == Cell::GUARD {
                break;
            } else {
                grid[row][c] = Cell::GUARDED;
            }
        }
    }
}

struct Input {
    m: i32,
    n: i32,
    guards: Vec<Vec<i32>>,
    walls: Vec<Vec<i32>>,
}

fn main() {
    let inputs = vec![
        Input {
            m: 4,
            n: 6,
            guards: vec![vec![0, 0], vec![1, 1], vec![2, 3]],
            walls: vec![vec![0, 1], vec![2, 2], vec![1, 4]],
        },
        Input {
            m: 3,
            n: 3,
            guards: vec![vec![1, 1]],
            walls: vec![vec![0, 1], vec![1, 0], vec![2, 1], vec![1, 2]],
        },
    ];

    for input in inputs {
        let result = Solution::count_unguarded(input.m, input.n, input.guards, input.walls);
        println!("{result}");
    }
}

struct Solution;

impl Solution {
    pub fn satisfies_conditions(grid: Vec<Vec<i32>>) -> bool {
        let n = grid.len() - 1;
        let m = grid[0].len() - 1;

        for r in 0..=n {
            for c in 0..=m {
                let cell = grid[r][c];
                let right = if c != m { grid[r][c + 1] } else { cell + 1 };
                let bottom = if r != n { grid[r + 1][c] } else { cell };

                if (cell != bottom) || (cell == right) {
                    return false;
                }
            }
        }

        true
    }
}

struct Input {
    grid: Vec<Vec<i32>>,
}

fn main() {
    let inputs = [
        Input {
            grid: [[1, 0, 2], [1, 0, 2]].map(|v| v.to_vec()).to_vec(),
        },
        Input {
            grid: [[1, 1, 1], [0, 0, 0]].map(|v| v.to_vec()).to_vec(),
        },
        Input {
            grid: [[1], [2], [3]].map(|v| v.to_vec()).to_vec(),
        },
    ];

    for input in inputs {
        let result = Solution::satisfies_conditions(input.grid);
        println!("{:?}", result);
    }
}

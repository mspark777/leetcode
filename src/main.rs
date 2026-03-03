struct Solution;

impl Solution {
    pub fn minimum_operations(grid: Vec<Vec<i32>>) -> i32 {
        let mut result = 0;
        let mut grid = grid;
        let n = grid.len();
        let m = grid[0].len();

        #[allow(clippy::needless_range_loop)]
        for c in 0..m {
            for r in 1..n {
                let cur = grid[r][c];
                let next = grid[r - 1][c] + 1;
                if cur < next {
                    result += next - cur;
                    grid[r][c] = next;
                }
            }
        }

        result
    }
}

struct Input {
    grid: Vec<Vec<i32>>,
}

fn main() {
    let inputs = [
        Input {
            grid: [[3, 2], [1, 3], [3, 4], [0, 1]]
                .map(|v| v.to_vec())
                .to_vec(),
        },
        Input {
            grid: [[3, 2, 1], [2, 1, 0], [1, 2, 3]]
                .map(|v| v.to_vec())
                .to_vec(),
        },
    ];

    for input in inputs {
        let result = Solution::minimum_operations(input.grid);
        println!("{:?}", result);
    }
}

struct Solution;

impl Solution {
    pub fn max_increase_keeping_skyline(grid: Vec<Vec<i32>>) -> i32 {
        let n = grid.len();
        let mut vertical = vec![0; n];
        let mut horizontal = vec![0; n];

        for (row, h) in grid.iter().zip(horizontal.iter_mut()) {
            for (cell, v) in row.iter().copied().zip(vertical.iter_mut()) {
                *h = cell.max(*h);
                *v = cell.max(*v);
            }
        }

        let mut result: i32 = 0;
        for (row, h) in grid.iter().zip(horizontal) {
            for (cell, v) in row.iter().copied().zip(vertical.iter().copied()) {
                if cell < h && cell < v {
                    result += h.min(v) - cell;
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
    let inputs = [Input {
        grid: [[3, 0, 8, 4], [2, 4, 5, 7], [9, 2, 6, 3], [0, 3, 1, 0]]
            .map(|v| v.to_vec())
            .to_vec(),
    }];

    for input in inputs {
        let result = Solution::max_increase_keeping_skyline(input.grid);
        println!("{:?}", result);
    }
}

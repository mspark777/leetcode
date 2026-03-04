struct Solution;

impl Solution {
    pub fn zigzag_traversal(grid: Vec<Vec<i32>>) -> Vec<i32> {
        let mut result = Vec::<i32>::new();

        for (i, row) in grid.iter().enumerate() {
            if (i & 1) == 0 {
                for (j, n) in row.iter().copied().enumerate() {
                    if (j & 1) == 0 {
                        result.push(n);
                    }
                }
            } else {
                for (j, n) in row.iter().copied().enumerate().rev() {
                    if (j & 1) == 1 {
                        result.push(n);
                    }
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
            grid: [[1, 2], [3, 4]].map(|v| v.to_vec()).to_vec(),
        },
        Input {
            grid: [[2, 1], [2, 1], [2, 1]].map(|v| v.to_vec()).to_vec(),
        },
        Input {
            grid: [[1, 2, 3], [4, 5, 6], [7, 8, 9]]
                .map(|v| v.to_vec())
                .to_vec(),
        },
    ];

    for input in inputs {
        let result = Solution::zigzag_traversal(input.grid);
        println!("{:?}", result);
    }
}

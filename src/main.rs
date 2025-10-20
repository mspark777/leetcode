struct Solution {}

impl Solution {
    pub fn check_x_matrix(grid: Vec<Vec<i32>>) -> bool {
        let n = grid.len();
        let last = n - 1;

        for (r, row) in grid.iter().enumerate() {
            for (c, cell) in row.iter().cloned().enumerate() {
                if (r == c) || (c == (last - r)) {
                    if cell == 0 {
                        return false;
                    }
                } else if cell != 0 {
                    return false;
                }
            }
        }

        return true;
    }
}

struct Input {
    grid: Vec<Vec<i32>>,
}

fn main() {
    let inputs = [
        Input {
            grid: vec![
                vec![2, 0, 0, 1],
                vec![0, 3, 1, 0],
                vec![0, 5, 2, 0],
                vec![4, 0, 0, 2],
            ],
        },
        Input {
            grid: vec![vec![5, 7, 0], vec![0, 3, 1], vec![0, 5, 0]],
        },
    ];

    for input in inputs {
        let result = Solution::check_x_matrix(input.grid);
        println!("{:?}", result);
    }
}

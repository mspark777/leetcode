struct Solution {}

impl Solution {
    pub fn delete_greatest_value(grid: Vec<Vec<i32>>) -> i32 {
        let mut grid = grid;
        for row in grid.iter_mut() {
            row.sort_unstable();
        }

        let mut result = 0;
        for i in 0..grid[0].len() {
            let mut m = 0;
            for row in grid.iter() {
                m = m.max(row[i]);
            }

            result += m;
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
            grid: [[1, 2, 4], [3, 3, 1]].map(|r| r.to_vec()).to_vec(),
        },
        Input {
            grid: [[10]].map(|r| r.to_vec()).to_vec(),
        },
    ];

    for input in inputs {
        let result = Solution::delete_greatest_value(input.grid);
        println!("{:?}", result);
    }
}

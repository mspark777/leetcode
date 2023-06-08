mod utils;

use utils::Solution;

impl Solution {
    pub fn count_negatives(grid: Vec<Vec<i32>>) -> i32 {
        let mut result = 0;
        let col_count = grid[0].len() as i32;
        let mut cur = col_count - 1;

        for row in grid.iter() {
            while cur >= 0 {
                if row[cur as usize] < 0 {
                    cur -= 1;
                } else {
                    break;
                }
            }

            result += col_count - (cur + 1)
        }

        return result;
    }
}

fn main() {
    let inputs = [
        vec![
            vec![4, 3, 2, -1],
            vec![3, 2, 1, -1],
            vec![1, 1, -1, -2],
            vec![-1, -1, -2, -3],
        ],
        vec![vec![3, 2], vec![1, 0]],
    ];

    for grid in inputs {
        let result = Solution::count_negatives(grid);
        println!("{result}");
    }
}

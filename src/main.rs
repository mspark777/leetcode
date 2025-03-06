struct Solution {}

impl Solution {
    pub fn find_missing_and_repeated_values(grid: Vec<Vec<i32>>) -> Vec<i32> {
        let mut sum = 0i64;
        let mut sqr_sum = 0i64;
        let n = grid.len();

        for row in 0..n {
            for col in 0..n {
                let num = grid[row][col] as i64;
                sum += num;
                sqr_sum += num * num;
            }
        }

        let total = (n * n) as i64;
        let sum_diff = sum - total * (total + 1) / 2;
        let sqr_diff = sqr_sum - total * (total + 1) * (2 * total + 1) / 6;
        let repeat = (sqr_diff / sum_diff + sum_diff) / 2;
        let missing = (sqr_diff / sum_diff - sum_diff) / 2;

        return vec![repeat as i32, missing as i32];
    }
}

struct Input {
    grid: Vec<Vec<i32>>,
}

fn main() {
    let inputs = vec![
        Input {
            grid: vec![vec![1, 3], vec![2, 2]],
        },
        Input {
            grid: vec![vec![9, 1, 7], vec![8, 9, 2], vec![3, 4, 6]],
        },
    ];

    for input in inputs {
        let result = Solution::find_missing_and_repeated_values(input.grid);
        println!("{result:?}");
    }
}

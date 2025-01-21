struct Solution {}

impl Solution {
    pub fn grid_game(grid: Vec<Vec<i32>>) -> i64 {
        let mut first_row_sum = 0i64;
        let mut second_row_sum = 0i64;
        let mut result = i64::MAX;

        for point in grid[0].iter().cloned() {
            first_row_sum += point as i64;
        }

        for (&first, &second) in grid[0].iter().zip(grid[1].iter()) {
            first_row_sum -= first as i64;
            result = result.min(first_row_sum.max(second_row_sum));
            second_row_sum += second as i64;
        }

        return result;
    }
}

struct Input {
    grid: Vec<Vec<i32>>,
}

fn main() {
    let inputs = vec![
        Input {
            grid: vec![vec![2, 5, 4], vec![1, 5, 1]],
        },
        Input {
            grid: vec![vec![3, 3, 1], vec![8, 5, 2]],
        },
        Input {
            grid: vec![vec![1, 3, 1, 15], vec![1, 3, 3, 1]],
        },
    ];

    for input in inputs {
        let result = Solution::grid_game(input.grid);
        println!("{result:?}");
    }
}

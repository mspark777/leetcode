struct Solution {}

impl Solution {
    pub fn find_column_width(grid: Vec<Vec<i32>>) -> Vec<i32> {
        let n = grid[0].len();

        let mut result = vec![0; n];
        for i in 0..n {
            let mut max_width = 0;
            for row in grid.iter() {
                max_width = max_width.max(Self::width(row[i]));
            }

            result[i] = max_width;
        }

        result
    }

    fn width(mut n: i32) -> i32 {
        if n == 0 {
            return 1;
        }

        let mut l = 0;
        if n < 0 {
            l += 1;
            n *= -1;
        }

        while n > 0 {
            n /= 10;
            l += 1;
        }

        l
    }
}

struct Input {
    grid: Vec<Vec<i32>>,
}

fn main() {
    let inputs = [
        Input {
            grid: [[1], [22], [333]].map(|a| a.to_vec()).to_vec(),
        },
        Input {
            grid: [[-15, 1, 3], [15, 7, 12], [5, 6, -2]]
                .map(|a| a.to_vec())
                .to_vec(),
        },
    ];

    for input in inputs {
        let result = Solution::find_column_width(input.grid);
        println!("{:?}", result);
    }
}

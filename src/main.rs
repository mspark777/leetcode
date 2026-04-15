struct Solution;

impl Solution {
    pub fn maximal_square(matrix: Vec<Vec<char>>) -> i32 {
        if matrix.is_empty() {
            return 0;
        }

        let n = matrix[0].len();
        let mut dp = vec![0; n];
        let mut size = 0;
        let mut pre_cell = 0;
        for (r, row) in matrix.iter().enumerate() {
            for (c, cell) in row.iter().copied().enumerate() {
                let cell = ((cell as u8) - b'0') as i32;
                let temp = dp[c];

                if (r == 0) || (c == 0) || (cell == 0) {
                    dp[c] = cell
                } else {
                    dp[c] = pre_cell.min(dp[c].min(dp[c - 1])) + 1;
                }

                size = dp[c].max(size);
                pre_cell = temp;
            }
        }

        size * size
    }
}

struct Input {
    matrix: Vec<Vec<char>>,
}

fn main() {
    let inputs = [Input {
        matrix: [
            ['1', '0', '1', '0', '0'],
            ['1', '0', '1', '1', '1'],
            ['1', '1', '1', '1', '1'],
            ['1', '0', '0', '1', '0'],
        ]
        .map(|v| v.to_vec())
        .to_vec(),
    }];

    for input in inputs {
        let result = Solution::maximal_square(input.matrix);
        println!("{:?}", result);
    }
}

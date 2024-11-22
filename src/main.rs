use std::collections::HashMap;

struct Solution {}

impl Solution {
    pub fn max_equal_rows_after_flips(matrix: Vec<Vec<i32>>) -> i32 {
        let mut pattern_frequencies = HashMap::<String, i32>::new();

        for row in matrix.iter() {
            let mut patterns = Vec::<char>::with_capacity(row.len());
            let first = row[0];
            for &cell in row.iter() {
                if first == cell {
                    patterns.push('t');
                } else {
                    patterns.push('f');
                }
            }

            let pattern = patterns.iter().collect::<String>();
            let count = pattern_frequencies.entry(pattern).or_insert(0);
            *count += 1;
        }

        let mut result = 0;
        for &count in pattern_frequencies.values() {
            result = result.max(count);
        }

        return result;
    }
}

struct Input {
    matrix: Vec<Vec<i32>>,
}

fn main() {
    let inputs = vec![
        Input {
            matrix: vec![vec![0, 1], vec![1, 1]],
        },
        Input {
            matrix: vec![vec![0, 1], vec![1, 0]],
        },
        Input {
            matrix: vec![vec![0, 0, 0], vec![0, 0, 1], vec![1, 1, 0]],
        },
    ];

    for input in inputs {
        let result = Solution::max_equal_rows_after_flips(input.matrix);
        println!("{result}");
    }
}

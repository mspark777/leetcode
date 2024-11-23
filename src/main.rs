struct Solution {}

impl Solution {
    pub fn rotate_the_box(matrix: Vec<Vec<char>>) -> Vec<Vec<char>> {
        let empty = '.';
        let stone = '#';
        let obstacle = '*';

        let m = matrix.len();
        let n = matrix[0].len();
        let mut result = vec![vec![empty; m]; n];

        for (r, row) in matrix.iter().enumerate() {
            let mut lowest_row_with_empty_cell = n - 1;
            for (c, &cell) in row.iter().enumerate().rev() {
                if cell == stone {
                    result[lowest_row_with_empty_cell][m - r - 1] = stone;
                    if lowest_row_with_empty_cell > 0 {
                        lowest_row_with_empty_cell -= 1;
                    }
                } else if cell == obstacle {
                    result[c][m - r - 1] = obstacle;
                    if c > 0 {
                        lowest_row_with_empty_cell = c - 1;
                    }
                }
            }
        }

        return result;
    }
}

struct Input {
    matrix: Vec<Vec<char>>,
}

fn main() {
    let inputs = vec![
        Input {
            matrix: vec![vec!['#', '.', '#']],
        },
        Input {
            matrix: vec![vec!['#', '.', '*', '.'], vec!['#', '#', '*', '.']],
        },
        Input {
            matrix: vec![
                vec!['#', '#', '*', '.', '*', '.'],
                vec!['#', '#', '#', '*', '.', '.'],
                vec!['#', '#', '#', '.', '#', '.'],
            ],
        },
    ];

    for input in inputs {
        let result = Solution::rotate_the_box(input.matrix);
        println!("{result:?}");
    }
}

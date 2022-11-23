use std::collections::HashSet;

struct Solution {}
impl Solution {
    pub fn is_valid_sudoku(board: Vec<Vec<char>>) -> bool {
        let mut seens = HashSet::<String>::new();
        for r in 0..9usize {
            for c in 0..9usize {
                let n = board[r][c];
                if n == '.' {
                    continue;
                }

                let ns = format!("({n})");
                let row = format!("{ns}{r}");
                let col = format!("{c}{ns}");

                let cross = format!("{}{ns}{}", r / 3, c / 3);
                if seens.contains(&row) || seens.contains(&col) || seens.contains(&cross) {
                    return false;
                }

                seens.insert(row);
                seens.insert(col);
                seens.insert(cross);
            }
        }

        return true;
    }
}

fn main() {
    let inputs = [
        [
            ['5', '3', '.', '.', '7', '.', '.', '.', '.'],
            ['6', '.', '.', '1', '9', '5', '.', '.', '.'],
            ['.', '9', '8', '.', '.', '.', '.', '6', '.'],
            ['8', '.', '.', '.', '6', '.', '.', '.', '3'],
            ['4', '.', '.', '8', '.', '3', '.', '.', '1'],
            ['7', '.', '.', '.', '2', '.', '.', '.', '6'],
            ['.', '6', '.', '.', '.', '.', '2', '8', '.'],
            ['.', '.', '.', '4', '1', '9', '.', '.', '5'],
            ['.', '.', '.', '.', '8', '.', '.', '7', '9'],
        ],
        [
            ['8', '3', '.', '.', '7', '.', '.', '.', '.'],
            ['6', '.', '.', '1', '9', '5', '.', '.', '.'],
            ['.', '9', '8', '.', '.', '.', '.', '6', '.'],
            ['8', '.', '.', '.', '6', '.', '.', '.', '3'],
            ['4', '.', '.', '8', '.', '3', '.', '.', '1'],
            ['7', '.', '.', '.', '2', '.', '.', '.', '6'],
            ['.', '6', '.', '.', '.', '.', '2', '8', '.'],
            ['.', '.', '.', '4', '1', '9', '.', '.', '5'],
            ['.', '.', '.', '.', '8', '.', '.', '7', '9'],
        ],
    ];

    for board in inputs {
        let result = Solution::is_valid_sudoku(board.iter().map(|r| r.to_vec()).collect());
        println!("{result}");
    }
}

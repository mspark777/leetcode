struct Solution {}
impl Solution {
    pub fn exist(board: Vec<Vec<char>>, word: String) -> bool {
        let mut board = board;
        let first = word.chars().nth(0).unwrap();
        for r in 0..board.len() {
            for c in 0..board[r].len() {
                if board[r][c] == first {
                    if Self::dfs(&mut board, r as i32, c as i32, word.as_str()) {
                        return true;
                    }
                }
            }
        }

        return false;
    }

    fn dfs(board: &mut Vec<Vec<char>>, row: i32, col: i32, chars: &str) -> bool {
        if let Some(ch) = chars.chars().nth(0) {
            if (row < 0) || (col < 0) {
                return false;
            }

            let r = row as usize;
            let c = col as usize;
            if r >= board.len() {
                return false;
            } else if c >= board[0].len() {
                return false;
            }

            if board[r][c] != ch {
                return false;
            }

            board[r][c] = 0 as char;

            let next = &chars[1..];
            let found = Self::dfs(board, row + 1, col, next)
                || Self::dfs(board, row - 1, col, next)
                || Self::dfs(board, row, col + 1, next)
                || Self::dfs(board, row, col - 1, next);

            board[r][c] = ch;

            return found;
        } else {
            return true;
        }
    }
}

struct Input {
    board: Vec<Vec<char>>,
    word: String,
}
fn main() {
    let inputs = [
        Input {
            board: vec![
                vec!['A', 'B', 'C', 'E'],
                vec!['S', 'F', 'C', 'S'],
                vec!['A', 'D', 'E', 'E'],
            ],
            word: "ABCCED".to_string(),
        },
        Input {
            board: vec![
                vec!['A', 'B', 'C', 'E'],
                vec!['S', 'F', 'C', 'S'],
                vec!['A', 'D', 'E', 'E'],
            ],
            word: "SEE".to_string(),
        },
        Input {
            board: vec![
                vec!['A', 'B', 'C', 'E'],
                vec!['S', 'F', 'C', 'S'],
                vec!['A', 'D', 'E', 'E'],
            ],
            word: "ABCB".to_string(),
        },
        Input {
            board: vec![
                vec!['A', 'A', 'A', 'A', 'A', 'A'],
                vec!['A', 'A', 'A', 'A', 'A', 'A'],
                vec!['A', 'A', 'A', 'A', 'A', 'A'],
                vec!['A', 'A', 'A', 'A', 'A', 'A'],
                vec!['A', 'A', 'A', 'A', 'A', 'A'],
                vec!['A', 'A', 'A', 'A', 'A', 'A'],
            ],
            word: "AAAAAAAAAAAABAA".to_string(),
        },
        Input {
            board: vec![vec!['a']],
            word: "a".to_string(),
        },
    ];

    for Input { board, word } in inputs {
        let result = Solution::exist(board, word);
        println!("{result}");
    }
}

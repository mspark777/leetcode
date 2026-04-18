struct Solution;

impl Solution {
    pub fn game_of_life(board: &mut [Vec<i32>]) {
        let m = board.len() as i32;
        let n = board[0].len() as i32;

        for i in 0..m {
            for j in 0..n {
                let lives = Self::live_neighbors(board, i, j, m, n);
                let i = i as usize;
                let j = j as usize;
                if (board[i][j] == 1) && (2..=3).contains(&lives) {
                    board[i][j] = 3;
                }

                if (board[i][j] == 0) && (lives == 3) {
                    board[i][j] = 2;
                }
            }
        }

        for i in 0..m {
            for j in 0..n {
                board[i as usize][j as usize] >>= 1;
            }
        }
    }

    fn live_neighbors(board: &[Vec<i32>], i: i32, j: i32, m: i32, n: i32) -> i32 {
        let mut lives = 0;

        for x in 0.max(i - 1)..=(i + 1).min(m - 1) {
            for y in 0.max(j - 1)..=(j + 1).min(n - 1) {
                lives += board[x as usize][y as usize] & 1;
            }
        }
        lives -= board[i as usize][j as usize] & 1;
        lives
    }
}

struct Input {
    board: Vec<Vec<i32>>,
}

fn main() {
    let mut inputs = [
        Input {
            board: [[0, 1, 0], [0, 0, 1], [1, 1, 1], [0, 0, 0]]
                .map(|v| v.to_vec())
                .to_vec(),
        },
        Input {
            board: [[1, 1], [1, 0]].map(|v| v.to_vec()).to_vec(),
        },
    ];

    for input in inputs.iter_mut() {
        Solution::game_of_life(&mut input.board);
        println!("{:?}", input.board);
    }
}

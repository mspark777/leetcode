struct Solution;

impl Solution {
    pub fn valid_tic_tac_toe(board: Vec<String>) -> bool {
        let mut count = [0, 0];
        let mut arr = [[0; 8]; 2];
        for (i, s) in board.iter().enumerate() {
            for (j, b) in s.bytes().enumerate() {
                let idx = match b {
                    b'X' => 0,
                    b'O' => 1,
                    _ => 2,
                };

                if idx != 2 {
                    count[idx] += 1;
                    arr[idx][i] += 1;
                    arr[idx][j + 3] += 1;
                    if i == j {
                        arr[idx][6] += 1;
                    }
                    if i == (2 - j) {
                        arr[idx][7] += 1;
                    }
                }
            }
        }

        let mut win = [false, false];
        for (i, c) in arr.iter().enumerate() {
            if c.contains(&3) {
                win[i] = true;
            }
        }

        if win[0] && win[1] {
            return false;
        }

        (count[0] == count[1] && !win[0]) || (count[0] == count[1] + 1 && !win[1])
    }
}

struct Input {
    board: Vec<String>,
}

fn main() {
    let inputs = [Input {
        board: ["O  ", "   ", "   "].map(|v| v.to_string()).to_vec(),
    }];

    for input in inputs {
        let result = Solution::valid_tic_tac_toe(input.board);
        println!("{:?}", result);
    }
}

struct Solution {}

impl Solution {
    pub fn snakes_and_ladders(board: Vec<Vec<i32>>) -> i32 {
        let n = board.len();
        let mut min_rolls = vec![-1; n * n + 1];
        let mut queue = std::collections::VecDeque::<i32>::new();

        min_rolls[1] = 0;
        queue.push_back(1);

        while let Some(x) = queue.pop_front() {
            for i in 1..=6 {
                let n = n as i32;
                if (x + i) > (n * n) {
                    break;
                }

                let t = x + i;
                let row = (t - 1) / n;
                let col = (t - 1) % n;
                let v = if row & 1 == 1 {
                    board[(n - 1 - row) as usize][(n - 1 - col) as usize]
                } else {
                    board[(n - 1 - row) as usize][col as usize]
                };

                let y = if v > 0 { v } else { t };

                if y == n * n {
                    return min_rolls[x as usize] + 1;
                }

                if min_rolls[y as usize] == -1 {
                    min_rolls[y as usize] = min_rolls[x as usize] + 1;
                    queue.push_back(y);
                }
            }
        }

        return -1;
    }
}

struct Input {
    board: Vec<Vec<i32>>,
}

fn main() {
    let inputs = vec![
        Input {
            board: vec![
                vec![-1, -1, -1, -1, -1, -1],
                vec![-1, -1, -1, -1, -1, -1],
                vec![-1, -1, -1, -1, -1, -1],
                vec![-1, 35, -1, -1, 13, -1],
                vec![-1, -1, -1, -1, -1, -1],
                vec![-1, 15, -1, -1, -1, -1],
            ],
        },
        Input {
            board: vec![vec![-1, -1], vec![-1, 3]],
        },
        Input {
            board: vec![
                vec![-1, 1, 2, -1],
                vec![2, 13, 15, -1],
                vec![-1, 10, -1, -1],
                vec![-1, 6, 2, 8],
            ],
        },
        Input {
            board: vec![
                vec![-1, -1, 27, 13, -1, 25, -1],
                vec![-1, -1, -1, -1, -1, -1, -1],
                vec![44, -1, 8, -1, -1, 2, -1],
                vec![-1, 30, -1, -1, -1, -1, -1],
                vec![3, -1, 20, -1, 46, 6, -1],
                vec![-1, -1, -1, -1, -1, -1, 29],
                vec![-1, 29, 21, 33, -1, -1, -1],
            ],
        },
    ];

    for input in inputs {
        let result = Solution::snakes_and_ladders(input.board);
        println!("{:?}", result);
    }
}

use std::collections::VecDeque;

struct Solution {}
impl Solution {
    pub fn nearest_exit(maze: Vec<Vec<char>>, entrance: Vec<i32>) -> i32 {
        let mut maze = maze;
        const WALL: char = '+';
        let row_count = maze.len() as i32;
        let col_count = maze[0].len() as i32;
        let last_row = row_count - 1;
        let last_col = col_count - 1;
        let dirs = [[1, 0], [-1, 0], [0, 1], [0, -1]];

        let mut queue = VecDeque::<[i32; 3]>::new();
        queue.push_back([entrance[0], entrance[1], 0]);

        maze[entrance[0] as usize][entrance[1] as usize] = WALL;
        while let Some([row, col, steps]) = queue.pop_front() {
            let next_steps = steps + 1;

            for [r, c] in dirs {
                let next_row = row + r;
                let next_col = col + c;
                if next_row < 0 {
                    continue;
                } else if next_row >= row_count {
                    continue;
                } else if next_col < 0 {
                    continue;
                } else if next_col >= col_count {
                    continue;
                } else if maze[next_row as usize][next_col as usize] == WALL {
                    continue;
                }

                if next_row == 0 {
                    return next_steps;
                } else if next_row == last_row {
                    return next_steps;
                } else if next_col == 0 {
                    return next_steps;
                } else if next_col == last_col {
                    return next_steps;
                }

                maze[next_row as usize][next_col as usize] = WALL;
                queue.push_back([next_row, next_col, next_steps]);
            }
        }

        return -1;
    }
}

struct Input {
    maze: Vec<Vec<char>>,
    entrance: Vec<i32>,
}

fn main() {
    let inputs = [
        Input {
            maze: vec![
                vec!['+', '+', '.', '+'],
                vec!['.', '.', '.', '+'],
                vec!['+', '+', '+', '.'],
            ],
            entrance: vec![1, 2],
        },
        Input {
            maze: vec![
                vec!['+', '+', '+'],
                vec!['.', '.', '.'],
                vec!['+', '+', '+'],
            ],
            entrance: vec![1, 0],
        },
        Input {
            maze: vec![vec!['.', '+']],
            entrance: vec![0, 0],
        },
    ];

    for Input { maze, entrance } in inputs {
        let result = Solution::nearest_exit(maze, entrance);
        println!("{result}");
    }
}

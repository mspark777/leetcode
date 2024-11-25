use std::collections::{HashSet, VecDeque};

struct Solution {}

impl Solution {
    pub fn sliding_puzzle(board: Vec<Vec<i32>>) -> i32 {
        let directions = vec![
            vec![1, 3],
            vec![0, 2, 4],
            vec![1, 5],
            vec![0, 4],
            vec![1, 3, 5],
            vec![2, 4],
        ];
        let target = "123450".to_string();
        let start_state = board
            .iter()
            .flatten()
            .map(|&x| x.to_string())
            .collect::<String>();

        let mut visited = HashSet::<String>::new();
        let mut queue = VecDeque::<String>::new();
        queue.push_back(start_state.clone());
        visited.insert(start_state);

        let mut result = 0;
        while !queue.is_empty() {
            let mut queue_size = queue.len();
            while queue_size > 0 {
                queue_size -= 1;
                let current_state = queue.pop_front().unwrap();
                if current_state == target {
                    return result;
                }

                let zero_pos = current_state.chars().position(|c| c == '0').unwrap();

                for &new_pos in directions[zero_pos].iter() {
                    let mut next_state = current_state.clone().chars().collect::<Vec<char>>();
                    let new_char = next_state[new_pos];
                    let zero_char = next_state[zero_pos];
                    next_state[zero_pos] = new_char;
                    next_state[new_pos] = zero_char;

                    let next_str = next_state.iter().collect::<String>();
                    if visited.contains(&next_str) {
                        continue;
                    }

                    visited.insert(next_str.clone());
                    queue.push_back(next_str);
                }
            }

            result += 1;
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
            board: vec![vec![1, 2, 3], vec![4, 0, 5]],
        },
        Input {
            board: vec![vec![1, 2, 3], vec![5, 4, 0]],
        },
        Input {
            board: vec![vec![4, 1, 2], vec![5, 0, 3]],
        },
    ];

    for input in inputs {
        let result = Solution::sliding_puzzle(input.board);
        println!("{result}");
    }
}

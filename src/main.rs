struct Solution {}

impl Solution {
    pub fn push_dominoes(dominoes: String) -> String {
        const LEFT: char = 'L';
        const RIGHT: char = 'R';
        const STAND: char = '.';

        let dominoes = dominoes.chars().collect::<Vec<char>>();
        let dominoes_len = dominoes.len();
        let max_force = dominoes_len as i32;

        let mut force = 0;
        let mut forces = vec![0; dominoes_len];
        for (i, domino) in dominoes.iter().cloned().enumerate() {
            if domino == LEFT {
                force = 0;
            } else if domino == RIGHT {
                force = max_force;
            } else {
                force = force.max(1) - 1;
            }

            forces[i] += force;
        }

        force = 0;
        for (i, domino) in dominoes.iter().cloned().enumerate().rev() {
            if domino == LEFT {
                force = max_force;
            } else if domino == RIGHT {
                force = 0;
            } else {
                force = force.max(1) - 1;
            }

            forces[i] -= force;
        }

        let mut result = vec![' '; dominoes_len];
        for (i, force) in forces.into_iter().enumerate() {
            if force < 0 {
                result[i] = LEFT;
            } else if force > 0 {
                result[i] = RIGHT;
            } else {
                result[i] = STAND;
            }
        }

        return result.iter().collect();
    }
}

struct Input {
    dominoes: String,
}

fn main() {
    let inputs = vec![
        Input {
            dominoes: "RR.L".to_string(),
        },
        Input {
            dominoes: ".L.R...LR..L..".to_string(),
        },
    ];

    for input in inputs {
        let result = Solution::push_dominoes(input.dominoes);
        println!("{result:?}");
    }
}

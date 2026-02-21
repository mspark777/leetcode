struct Solution;

impl Solution {
    pub fn final_position_of_snake(n: i32, commands: Vec<String>) -> i32 {
        let mut r = 0;
        let mut c = 0;
        for command in commands {
            if command == "UP" {
                r -= 1;
            } else if command == "DOWN" {
                r += 1;
            } else if command == "RIGHT" {
                c += 1;
            } else if command == "LEFT" {
                c -= 1;
            }
        }

        r * n + c
    }
}

struct Input {
    n: i32,
    commands: Vec<String>,
}

fn main() {
    let inputs = [
        Input {
            n: 2,
            commands: ["RIGHT", "DOWN"].map(|s| s.to_string()).to_vec(),
        },
        Input {
            n: 3,
            commands: ["DOWN", "RIGHT", "UP"].map(|s| s.to_string()).to_vec(),
        },
    ];

    for input in inputs {
        let result = Solution::final_position_of_snake(input.n, input.commands);
        println!("{:?}", result);
    }
}

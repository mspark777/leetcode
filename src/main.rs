struct Solution {}

impl Solution {
    pub fn interpret(command: String) -> String {
        command.clone().replace("()", "o").replace("(al)", "al")
    }
}

struct Input {
    command: String,
}

fn main() {
    let inputs = [
        Input {
            command: "G()(al)".to_string(),
        },
        Input {
            command: "G()()()()(al)".to_string(),
        },
        Input {
            command: "(al)G(al)()()G".to_string(),
        },
    ];

    for input in inputs {
        let result = Solution::interpret(input.command);
        println!("{:?}", result);
    }
}

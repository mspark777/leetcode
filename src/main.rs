struct Solution {}

impl Solution {
    pub fn final_value_after_operations(operations: Vec<String>) -> i32 {
        operations
            .iter()
            .fold(0, |result, oper| match oper.chars().nth(1) {
                Some('+') => result + 1,
                _ => result - 1,
            })
    }
}

struct Input {
    operations: Vec<String>,
}

fn main() {
    let inputs = [
        Input {
            operations: ["--X", "X++", "X++"].map(|s| s.to_string()).to_vec(),
        },
        Input {
            operations: ["++X", "++X", "X++"].map(|s| s.to_string()).to_vec(),
        },
        Input {
            operations: ["X++", "++X", "--X", "X--"].map(|s| s.to_string()).to_vec(),
        },
    ];

    for input in inputs {
        let result = Solution::final_value_after_operations(input.operations);
        println!("{:?}", result);
    }
}

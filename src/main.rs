mod solution;

use solution::Solution;

struct Input {
    s: String,
}

fn main() {
    let inputs: Vec<Input> = vec![
        Input {
            s: String::from("A man, a plan, a canal: Panama"),
        },
        Input {
            s: String::from("race a car"),
        },
        Input {
            s: String::from(" "),
        },
    ];

    for input in inputs {
        let result = Solution::is_palindrome(input.s);
        println!("{result:?}");
    }
}

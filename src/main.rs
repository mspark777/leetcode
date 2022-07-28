mod solution;

use solution::Solution;

struct Input {
    s: String,
    t: String,
}

fn main() {
    let inputs: Vec<Input> = vec![
        Input {
            s: String::from("anagram"),
            t: String::from("nagaram"),
        },
        Input {
            s: String::from("rat"),
            t: String::from("car"),
        },
    ];

    for input in inputs {
        let result = Solution::is_anagram(input.s, input.t);
        println!("{:?}", result);
    }
}

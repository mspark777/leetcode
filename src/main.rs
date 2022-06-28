mod solution;

use solution::Solution;

struct Input {
    s: &'static str,
}

fn main() {
    let inputs = [
        Input { s: "aab" },
        Input { s: "aaabbbcc" },
        Input { s: "ceabaacb" },
    ];

    for input in inputs {
        let result = Solution::min_deletions(String::from(input.s));
        println!("{result:?}");
    }
}

mod solution;

use solution::Solution;

struct Input {
    matchsticks: Vec<i32>,
}

fn main() {
    let inputs = [
        Input {
            matchsticks: vec![1, 1, 2, 2, 2],
        },
        Input {
            matchsticks: vec![3, 3, 3, 3, 4],
        },
        Input {
            matchsticks: vec![6, 6, 6, 6, 4, 2, 2],
        },
    ];

    for input in inputs {
        let result = Solution::makesquare(input.matchsticks);
        println!("{result:?}");
    }
}

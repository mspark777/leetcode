mod solution;

use solution::Solution;

struct Input {
    cost: Vec<i32>,
}

fn main() {
    let inputs = [
        Input {
            cost: vec![10, 15, 20],
        },
        Input {
            cost: vec![1, 100, 1, 1, 1, 100, 1, 1, 100, 1],
        },
    ];

    for input in inputs {
        let result = Solution::min_cost_climbing_stairs(input.cost);
        println!("{result:?}");
    }
}

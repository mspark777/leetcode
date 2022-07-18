mod solution;

use solution::Solution;

struct Input {
    prices: Vec<i32>,
}

fn main() {
    let inputs: Vec<Input> = vec![
        Input {
            prices: vec![7, 1, 5, 3, 6, 4],
        },
        Input {
            prices: vec![7, 6, 4, 3, 1],
        },
    ];

    for input in inputs {
        let result = Solution::max_profit(input.prices);
        println!("{result:?}");
    }
}

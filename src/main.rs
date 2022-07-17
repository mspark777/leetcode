mod solution;

use solution::Solution;

struct Input {
    n: i32,
    k: i32,
}

fn main() {
    let inputs = [Input { n: 3, k: 0 }, Input { n: 3, k: 1 }];

    for input in inputs {
        let result = Solution::k_inverse_pairs(input.n, input.k);
        println!("{result:?}");
    }
}

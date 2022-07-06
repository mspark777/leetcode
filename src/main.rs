mod solution;

use solution::Solution;

struct Input {
    n: i32,
}

fn main() {
    let inputs = [
        Input { n: 2 },
        Input { n: 3 },
        Input { n: 4 },
        Input { n: 9 },
    ];

    for input in inputs {
        let result = Solution::fib(input.n);
        println!("{result:?}");
    }
}

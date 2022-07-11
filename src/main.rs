mod solution;

use solution::Solution;

struct Input {
    num_rows: i32,
}

fn main() {
    let inputs = [Input { num_rows: 5 }, Input { num_rows: 1 }];

    for input in inputs {
        let result = Solution::generate(input.num_rows);
        println!("{result:?}");
    }
}

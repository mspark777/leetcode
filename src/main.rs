mod solution;

use solution::Solution;

struct Input {
    nums: Vec<i32>,
}

fn main() {
    let inputs = [
        Input {
            nums: vec![1, 2, 3],
        },
        Input {
            nums: vec![1, 10, 2, 9],
        },
    ];

    for input in inputs {
        let result = Solution::min_moves2(input.nums);
        println!("{result:?}");
    }
}

mod solution;

use solution::Solution;

struct Input {
    nums: Vec<i32>,
}

fn main() {
    let inputs = [
        Input {
            nums: vec![100, 4, 200, 1, 3, 2],
        },
        Input {
            nums: vec![0, 3, 7, 2, 5, 8, 4, 6, 0, 1],
        },
    ];

    for input in inputs {
        let result = Solution::longest_consecutive(input.nums);
        println!("{result:?}");
    }
}

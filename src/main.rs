mod solution;

use solution::Solution;

struct Input {
    nums: Vec<i32>,
    k: i32,
}

fn main() {
    let inputs = [
        Input {
            nums: vec![1, -1, -2, 4, -7, 3],
            k: 2,
        },
        Input {
            nums: vec![10, -5, -2, 4, 0, 3],
            k: 3,
        },
        Input {
            nums: vec![1, -5, -20, 4, -1, 3, -6, -3],
            k: 2,
        },
    ];

    for input in inputs {
        let result = Solution::max_result(input.nums, input.k);
        println!("{result:?}");
    }
}

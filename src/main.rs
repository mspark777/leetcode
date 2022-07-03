mod solution;

use solution::Solution;

struct Input {
    nums: Vec<i32>,
}

fn main() {
    let inputs = [
        Input {
            nums: vec![1, 7, 4, 9, 2, 5],
        },
        Input {
            nums: vec![1, 17, 5, 10, 13, 15, 10, 5, 16, 8],
        },
        Input {
            nums: vec![1, 2, 3, 4, 5, 6, 7, 8, 9],
        },
    ];

    for input in inputs {
        let result = Solution::wiggle_max_length(input.nums);
        println!("{result:?}");
    }
}

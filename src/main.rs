mod solution;

use solution::Solution;

struct Input {
    nums: Vec<i32>,
    target: i32,
}

fn main() {
    let inputs: Vec<Input> = vec![
        Input {
            nums: vec![5, 7, 7, 8, 8, 10],
            target: 8,
        },
        Input {
            nums: vec![5, 7, 7, 8, 8, 10],
            target: 6,
        },
        Input {
            nums: vec![],
            target: 0,
        },
        Input {
            nums: vec![1],
            target: 1,
        },
    ];

    for input in inputs {
        let result = Solution::search_range(input.nums, input.target);
        println!("{result:?}");
    }
}

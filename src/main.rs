mod solution;

use solution::Solution;

struct Input {
    nums: Vec<i32>,
}

fn main() {
    let inputs: Vec<Input> = vec![
        Input {
            nums: vec![1, 2, 3, 1],
        },
        Input {
            nums: vec![1, 2, 3, 4],
        },
        Input {
            nums: vec![1, 1, 1, 3, 3, 4, 3, 2, 4, 2],
        },
    ];

    for input in inputs {
        let result = Solution::contains_duplicate(input.nums);
        println!("{result:?}");
    }
}

mod solution;

use solution::Solution;

struct Input {
    nums: Vec<i32>,
}

fn main() {
    let inputs: Vec<Input> = vec![
        Input {
            nums: vec![5, 2, 6, 1],
        },
        Input { nums: vec![-1] },
        Input { nums: vec![-1, -1] },
    ];

    for input in inputs {
        let result = Solution::count_smaller(input.nums);
        println!("{result:?}");
    }
}

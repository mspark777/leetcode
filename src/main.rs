struct Solution;

impl Solution {
    pub fn min_moves(nums: Vec<i32>) -> i32 {
        let min = nums.iter().copied().min().unwrap();
        nums.into_iter().fold(0, |acc, num| acc + num - min)
    }
}

struct Input {
    nums: Vec<i32>,
}

fn main() {
    let inputs = [
        Input {
            nums: [1, 2, 3].to_vec(),
        },
        Input {
            nums: [1, 1, 1].to_vec(),
        },
    ];

    for input in inputs.into_iter() {
        let result = Solution::min_moves(input.nums);
        println!("{:?}", result);
    }
}

struct Solution;

impl Solution {
    pub fn min_operations(nums: Vec<i32>) -> i32 {
        let first = nums[0];
        nums.into_iter()
            .skip(1)
            .find(|&num| num != first)
            .map(|_| 1)
            .unwrap_or_default()
    }
}

struct Input {
    nums: Vec<i32>,
}

fn main() {
    let inputs = [
        Input {
            nums: [1, 2].to_vec(),
        },
        Input {
            nums: [5, 5, 5].to_vec(),
        },
    ];

    for input in inputs {
        let result = Solution::min_operations(input.nums);
        println!("{:?}", result);
    }
}

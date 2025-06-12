struct Solution {}

impl Solution {
    pub fn max_adjacent_distance(nums: Vec<i32>) -> i32 {
        let n = nums.len();
        let mut result = (nums[0] - nums[n - 1]).abs();

        for i in 1..n {
            let left = nums[i - 1];
            let right = nums[i];

            result = (left - right).abs().max(result);
        }

        return result;
    }
}

struct Input {
    nums: Vec<i32>,
}

fn main() {
    let inputs = vec![
        Input {
            nums: vec![1, 2, 4],
        },
        Input {
            nums: vec![-5, -10, -5],
        },
    ];

    for input in inputs {
        let result = Solution::max_adjacent_distance(input.nums);
        println!("{:?}", result);
    }
}

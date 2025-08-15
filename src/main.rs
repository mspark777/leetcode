struct Solution {}

impl Solution {
    pub fn min_start_value(nums: Vec<i32>) -> i32 {
        let mut result = 1;
        let mut sum = 0;

        for num in nums.iter().cloned() {
            sum += num;
            result = result.min(sum);
        }

        0.max(-result) + 1
    }
}

struct Input {
    nums: Vec<i32>,
}

fn main() {
    let inputs = [
        Input {
            nums: [-3, 2, -3, 4, 2].to_vec(),
        },
        Input {
            nums: [1, 2].to_vec(),
        },
        Input {
            nums: [1, -2, -3].to_vec(),
        },
    ];

    for input in inputs {
        let result = Solution::min_start_value(input.nums);
        println!("{:?}", result);
    }
}

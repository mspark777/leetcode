struct Solution;

impl Solution {
    pub fn max_operations(nums: Vec<i32>) -> i32 {
        let mut result = 1;
        let sum = nums[0] + nums[1];

        for i in (3..nums.len()).step_by(2) {
            let s = nums[i - 1] + nums[i];
            if sum == s {
                result += 1
            } else {
                break;
            }
        }

        result
    }
}

struct Input {
    nums: Vec<i32>,
}

fn main() {
    let inputs = [
        Input {
            nums: [3, 2, 1, 4, 5].to_vec(),
        },
        Input {
            nums: [1, 5, 3, 3, 4, 1, 3, 2, 2, 3].to_vec(),
        },
        Input {
            nums: [5, 3].to_vec(),
        },
    ];

    for input in inputs {
        let result = Solution::max_operations(input.nums);
        println!("{:?}", result);
    }
}

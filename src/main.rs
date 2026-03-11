struct Solution;

impl Solution {
    pub fn minimum_pair_removal(nums: Vec<i32>) -> i32 {
        let mut result = 0;
        let mut nums = nums;

        while nums.len() > 1 {
            let mut is_ascending = true;
            let mut min_sum = i32::MAX;
            let n = nums.len();
            let mut target_index = n;

            for i in 0..(n - 1) {
                let left = nums[i];
                let right = nums[i + 1];
                let sum = left + right;
                if left > right {
                    is_ascending = false;
                }
                if sum < min_sum {
                    min_sum = sum;
                    target_index = i;
                }
            }

            if is_ascending {
                break;
            }

            result += 1;
            nums[target_index] = min_sum;
            nums.remove(target_index + 1);
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
            nums: [5, 2, 3, 1].to_vec(),
        },
        Input {
            nums: [1, 2, 2].to_vec(),
        },
    ];

    for input in inputs {
        let result = Solution::minimum_pair_removal(input.nums);
        println!("{:?}", result);
    }
}

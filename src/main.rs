struct Solution {}

impl Solution {
    pub fn apply_operations(nums: Vec<i32>) -> Vec<i32> {
        let mut nums = nums.clone();
        for j in 1..nums.len() {
            let i = j - 1;

            if nums[i] != nums[j] {
                continue;
            } else if nums[i] == 0 {
                continue;
            } else {
                nums[i] *= 2;
                nums[j] = 0;
            }
        }

        let mut non_zero_idx = 0;
        for i in 0..nums.len() {
            if nums[i] != 0 {
                nums[non_zero_idx] = nums[i];
                non_zero_idx += 1;
            }
        }

        for i in non_zero_idx..nums.len() {
            nums[i] = 0;
        }

        return nums;
    }
}

struct Input {
    nums: Vec<i32>,
}

fn main() {
    let inputs = vec![
        Input {
            nums: vec![1, 2, 2, 1, 1, 0],
        },
        Input { nums: vec![0, 1] },
    ];

    for input in inputs {
        let result = Solution::apply_operations(input.nums);
        println!("{result:?}");
    }
}

struct Solution {}

impl Solution {
    pub fn min_operations(nums: Vec<i32>) -> i32 {
        let n = nums.len();
        let mut result = 0;
        let mut nums = nums.clone();

        for i in 0..=(n - 3) {
            if nums[i] != 0 {
                continue;
            }

            nums[i] = 1;
            nums[i + 1] = if nums[i + 1] == 0 { 1 } else { 0 };
            nums[i + 2] = if nums[i + 2] == 0 { 1 } else { 0 };
            result += 1;
        }

        if (nums[n - 1] == 0) || (nums[n - 2] == 0) {
            return -1;
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
            nums: vec![0, 1, 1, 1, 0, 0],
        },
        Input {
            nums: vec![0, 1, 1, 1],
        },
    ];

    for input in inputs {
        let result = Solution::min_operations(input.nums);
        println!("{result:?}");
    }
}

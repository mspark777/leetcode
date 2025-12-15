struct Solution {}

impl Solution {
    pub fn alternating_subarray(nums: Vec<i32>) -> i32 {
        let n = nums.len();
        let mut dp = -1;
        let mut result = -1;
        for i in 1..n {
            if (dp > 0) && (nums[i] == nums[i - 2]) {
                dp += 1;
            } else if nums[i] == (nums[i - 1] + 1) {
                dp = 2;
            } else {
                dp = -1;
            }

            result = result.max(dp);
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
            nums: [2, 3, 4, 3, 4].to_vec(),
        },
        Input {
            nums: [4, 5, 6].to_vec(),
        },
    ];

    for input in inputs {
        let result = Solution::alternating_subarray(input.nums);
        println!("{:?}", result);
    }
}

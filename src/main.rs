struct Solution {}

impl Solution {
    pub fn max_ascending_sum(nums: Vec<i32>) -> i32 {
        let mut result = nums[0];
        let mut sum = nums[0];
        for sub in nums.windows(2) {
            let left = sub[0];
            let right = sub[1];
            if left < right {
                sum += right;
            } else {
                sum = right;
            }

            result = result.max(sum);
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
            nums: vec![10, 20, 30, 5, 10, 50],
        },
        Input {
            nums: vec![10, 20, 30, 40, 50],
        },
        Input {
            nums: vec![12, 17, 15, 13, 10, 11, 12],
        },
    ];

    for input in inputs {
        let result = Solution::max_ascending_sum(input.nums);
        println!("{result:?}");
    }
}

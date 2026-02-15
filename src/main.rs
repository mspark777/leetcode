struct Solution;

impl Solution {
    pub fn minimum_average(nums: Vec<i32>) -> f64 {
        let mut nums = nums;
        nums.sort();

        let n = nums.len();
        let mut left = 0usize;
        let mut right = n - 1;
        let mut result = 101.0f64;

        while left < right {
            let l = nums[left] as f64;
            let r = nums[right] as f64;
            left += 1;
            right -= 1;

            result = result.min((l + r) / 2.0);
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
            nums: [7, 8, 3, 4, 15, 13, 4, 1].to_vec(),
        },
        Input {
            nums: [1, 9, 8, 3, 10, 5].to_vec(),
        },
        Input {
            nums: [1, 2, 3, 7, 8, 9].to_vec(),
        },
    ];

    for input in inputs {
        let result = Solution::minimum_average(input.nums);
        println!("{:?}", result);
    }
}

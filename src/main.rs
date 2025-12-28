struct Solution;

impl Solution {
    pub fn minimum_sum(nums: Vec<i32>) -> i32 {
        let n = nums.len();
        let mut left = vec![0; n];
        let mut right = vec![0; n];

        left[0] = nums[0];
        right[n - 1] = nums[n - 1];

        for i in 1..n {
            left[i] = left[i - 1].min(nums[i])
        }

        for i in (0..(n - 1)).rev() {
            right[i] = right[i + 1].min(nums[i])
        }

        const MAX_SUM: i32 = 2_000_000_000;
        let mut min_sum = MAX_SUM;

        for i in 1..(n - 1) {
            if (left[i] < nums[i]) && (right[i] < nums[i]) {
                min_sum = min_sum.min(left[i] + nums[i] + right[i]);
            }
        }

        match min_sum == MAX_SUM {
            true => -1,
            _ => min_sum,
        }
    }
}

struct Input {
    nums: Vec<i32>,
}

fn main() {
    let inputs = [
        Input {
            nums: [8, 6, 1, 5, 3].to_vec(),
        },
        Input {
            nums: [5, 4, 8, 7, 10, 2].to_vec(),
        },
        Input {
            nums: [6, 5, 4, 3, 4, 5].to_vec(),
        },
    ];

    for input in inputs {
        let result = Solution::minimum_sum(input.nums);
        println!("{:?}", result);
    }
}

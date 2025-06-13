struct Solution {}

impl Solution {
    pub fn minimize_max(nums: Vec<i32>, p: i32) -> i32 {
        let mut nums = nums;
        nums.sort();
        let n = nums.len();
        let mut left = 0;
        let mut right = nums[n - 1] - nums[0];
        while left < right {
            let mid = (left + right) / 2;
            if Self::count_valid_pairs(&nums, mid) >= p {
                right = mid;
            } else {
                left = mid + 1;
            }
        }

        return left;
    }

    fn count_valid_pairs(nums: &Vec<i32>, threshold: i32) -> i32 {
        let mut idx = 0usize;
        let mut count = 0;
        let n = nums.len();

        while idx < (n - 1) {
            let l = nums[idx];
            let r = nums[idx + 1];
            if (r - l) <= threshold {
                count += 1;
                idx += 1;
            }
            idx += 1;
        }

        return count;
    }
}

struct Input {
    nums: Vec<i32>,
    p: i32,
}

fn main() {
    let inputs = vec![
        Input {
            nums: vec![10, 1, 2, 7, 1, 3],
            p: 2,
        },
        Input {
            nums: vec![4, 2, 1, 2],
            p: 1,
        },
    ];

    for input in inputs {
        let result = Solution::minimize_max(input.nums, input.p);
        println!("{:?}", result);
    }
}

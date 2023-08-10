mod utils;

use utils::Solution;

impl Solution {
    pub fn search(nums: Vec<i32>, target: i32) -> bool {
        if nums.is_empty() {
            return false;
        }

        let mut left = 0usize;
        let mut right = nums.len();
        while left < right {
            let mid = (left + right) / 2;
            let mnum = nums[mid];
            if mnum == target {
                return true;
            }

            let lnum = nums[left];
            if lnum == mnum {
                left += 1;
                continue;
            }

            let pivot_array = lnum <= mnum;
            let target_array = lnum <= target;
            if pivot_array != target_array {
                if pivot_array {
                    left = mid + 1;
                } else {
                    right = mid;
                }
            } else {
                if mnum < target {
                    left = mid + 1;
                } else {
                    right = mid;
                }
            }
        }

        return false;
    }
}

fn main() {
    let inputs = [
        (vec![2, 5, 6, 0, 0, 1, 2], 0),
        (vec![2, 5, 6, 0, 0, 1, 2], 3),
        (vec![1, 0, 1, 1, 1], 0),
    ];

    for (nums, target) in inputs {
        let result = Solution::search(nums, target);
        println!("{result}");
    }
}

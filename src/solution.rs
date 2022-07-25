pub struct Solution {}
impl Solution {
    pub fn search_range(nums: Vec<i32>, target: i32) -> Vec<i32> {
        if nums.is_empty() {
            return vec![-1, -1];
        }

        let first = Self::search(&nums, target, true);
        let last = Self::search(&nums, target, false);

        vec![first, last]
    }

    fn search(nums: &Vec<i32>, target: i32, first: bool) -> i32 {
        let mut result = -1;
        let mut left = 0usize;
        let mut right = nums.len() - 1;

        while left <= right {
            let mid = (left + right) / 2;
            let num = nums[mid];
            if num > target {
                if mid > 0 {
                    right = mid - 1;
                } else {
                    break;
                }
            } else if num < target {
                left = mid + 1;
            } else {
                result = mid as i32;
                if first {
                    if mid > 0 {
                        right = mid - 1;
                    } else {
                        break;
                    }
                } else {
                    left = mid + 1;
                }
            }
        }

        result
    }
}

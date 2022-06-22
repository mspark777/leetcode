pub struct Solution {}

impl Solution {
    pub fn find_kth_largest(nums: Vec<i32>, k: i32) -> i32 {
        let mut nums = nums;
        let k = k as usize;
        let target = nums.len() - k;
        let mut left = 0;
        let mut right = nums.len() - 1;

        loop {
            let mid = Self::partition(&mut nums, left, right);
            if mid > target {
                right = mid - 1;
            } else if mid < target {
                left = mid + 1;
            } else {
                return nums[target];
            }
        }
    }

    fn partition(nums: &mut Vec<i32>, left: usize, right: usize) -> usize {
        let pivot = nums[right];
        let mut j = left;
        for i in left..right {
            if nums[i] < pivot {
                Self::swap(nums, i, j);
                j += 1;
            }
        }

        Self::swap(nums, j, right);
        j
    }

    fn swap(nums: &mut Vec<i32>, i: usize, j: usize) {
        let t = nums[i];
        nums[i] = nums[j];
        nums[j] = t;
    }
}

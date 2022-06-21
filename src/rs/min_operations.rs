pub struct MinOperations {}
impl MinOperations {
    pub fn min_operations(nums: Vec<i32>, x: i32) -> i32 {
        let mut target = -x;
        for n in nums.iter() {
            target += *n;
        }

        let nums_len = nums.len();
        if target < 0 {
            return -1;
        } else if target == 0 {
            return nums_len as i32;
        }

        let mut sum = 0;
        let mut result = -1;
        let mut left = 0usize;
        let mut right = 0usize;
        while right < nums_len {
            sum += nums[right];
            while sum > target {
                sum -= nums[left];
                left += 1;
            }

            if sum == target {
                let len = right - left + 1;
                result = result.max(len as i32);
            }

            right += 1;
        }

        if result != -1 {
            nums_len as i32 - result
        } else {
            -1
        }
    }
}

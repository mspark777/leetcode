pub struct MaximumUniqueSubarray {}
impl MaximumUniqueSubarray {
    pub fn maximum_unique_subarray(nums: Vec<i32>) -> i32 {
        use std::collections::HashMap;

        let mut result = i32::min_value();
        let mut sum = 0;
        let mut visits = HashMap::<i32, usize>::new();
        let mut left = 0usize;
        let mut right = 0usize;
        let nums_len = nums.len();
        while right < nums_len {
            let rn = nums[right];
            if let Some(visited) = visits.get(&rn) {
                while left <= *visited {
                    sum -= nums[left];
                    left += 1;
                }
            }

            sum += rn;
            visits.insert(rn, right);
            result = result.max(sum);
            right += 1;
        }

        result
    }
}

pub struct Solution {}

impl Solution {
    pub fn wiggle_max_length(nums: Vec<i32>) -> i32 {
        let nums_len = nums.len();
        if nums_len < 2 {
            return nums_len as i32;
        }

        let mut down = 1;
        let mut up = 1;
        for i in 1..nums_len {
            let cur = nums[i];
            let prev = nums[i - 1];
            if cur > prev {
                up = down + 1;
            } else if cur < prev {
                down = up + 1;
            }
        }

        up.max(down)
    }
}

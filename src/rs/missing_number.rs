#[allow(dead_code)]
pub struct MissingNumber {}

#[allow(dead_code)]
impl MissingNumber {
    pub fn missing_number(nums: Vec<i32>) -> i32 {
        let mut result = 0;
        let nums_len = nums.len();
        for i in 0..nums_len {
            let index = i as i32;
            result ^= index ^ nums[i]
        }

        result ^ (nums_len as i32)
    }
}

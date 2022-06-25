pub struct Solution {}

impl Solution {
    pub fn check_possibility(nums: Vec<i32>) -> bool {
        if nums.len() < 3 {
            return true;
        }

        let mut nums = nums;
        let mut count = 0;
        for i in 1..nums.len() {
            let prev = nums[i - 1];
            let cur = nums[i];
            if cur < prev {
                count += 1;
                if count > 1 {
                    return false;
                }

                if i > 1 {
                    let pprev = nums[i - 2];
                    if pprev <= cur {
                        nums[i - 1] = pprev;
                    } else {
                        nums[i] = prev;
                    }
                }
            }
        }

        count < 2
    }
}

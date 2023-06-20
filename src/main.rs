mod utils;

use utils::Solution;

impl Solution {
    pub fn get_averages(nums: Vec<i32>, k: i32) -> Vec<i32> {
        if k < 1 {
            return nums;
        }

        let mut result = vec![-1; nums.len()];
        let window_len = (k * 2) + 1;
        if (window_len as usize) > nums.len() {
            return result;
        }

        let mut window_sum = nums
            .iter()
            .take(window_len as usize)
            .cloned()
            .reduce(|acc, cur| acc + cur)
            .unwrap();
        result[k as usize] = window_sum / window_len;

        for i in (window_len as usize)..nums.len() {
            window_sum = window_sum - nums[i - window_len as usize] + nums[i];
            result[i - k as usize] = window_sum / window_len;
        }

        return result;
    }
}

fn main() {
    let inputs = [
        (vec![7, 4, 3, 9, 1, 8, 5, 2, 6], 3),
        (vec![100000], 0),
        (vec![8], 100000),
    ];

    for (nums, k) in inputs {
        let result = Solution::get_averages(nums, k);
        println!("{result:?}");
    }
}

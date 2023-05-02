mod utils;

use utils::Solution;

impl Solution {
    pub fn array_sign(nums: Vec<i32>) -> i32 {
        let mut sign = 0;
        for &num in nums.iter() {
            if num == 0 {
                return 0;
            }

            if num < 0 {
                sign ^= 1;
            }
        }

        return if sign == 0 { 1 } else { -1 };
    }
}

fn main() {
    let inputs = [
        vec![-1, -2, -3, -4, 3, 2, 1],
        vec![1, 5, 0, 2, -3],
        vec![-1, 1, -1, 1, -1],
        vec![9, 72, 34, 29, -49, -22, -77, -17, -66, -75, -44, -30, -24],
    ];

    for nums in inputs {
        let result = Solution::array_sign(nums);
        println!("{result}");
    }
}

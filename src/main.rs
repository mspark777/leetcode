mod utils;

use utils::Solution;

impl Solution {
    pub fn single_number(nums: Vec<i32>) -> i32 {
        let mut a = 0;
        let mut b = 0;

        for &num in nums.iter() {
            let tempa = (a & !b & !num) + (!a & b & num);
            let tempb = (!a & b & !num) + (!a & !b & num);

            a = tempa;
            b = tempb;
        }

        return a | b;
    }
}

fn main() {
    let inputs = [vec![2, 2, 3, 2], vec![0, 1, 0, 1, 0, 1, 99]];

    for nums in inputs {
        let result = Solution::single_number(nums);
        println!("{result}");
    }
}

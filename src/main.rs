mod utils;

use utils::Solution;

impl Solution {
    pub fn min_cost(nums: Vec<i32>, cost: Vec<i32>) -> i64 {
        let mut left = 1000001;
        let mut right = 0;
        for &num in nums.iter() {
            left = left.min(num);
            right = right.max(num);
        }

        let mut result = Self::get_cost(&nums, &cost, nums[0]);
        while left < right {
            let mid = (left + right) / 2;
            let cost1 = Self::get_cost(&nums, &cost, mid);
            let cost2 = Self::get_cost(&nums, &cost, mid + 1);

            if cost1 < cost2 {
                result = cost1;
                right = mid;
            } else {
                result = cost2;
                left = mid + 1;
            }
        }

        return result;
    }

    fn get_cost(nums: &Vec<i32>, cost: &Vec<i32>, base: i32) -> i64 {
        let mut result = 0i64;

        for (i, &num) in nums.iter().enumerate() {
            let diff = (num - base).abs() as i64;
            result += diff * (cost[i]) as i64;
        }

        return result;
    }
}

fn main() {
    let inputs = [
        (vec![1, 3, 5, 2], vec![2, 3, 1, 14]),
        (vec![2, 2, 2, 2, 2], vec![4, 2, 8, 1, 3]),
    ];

    for (nums, cost) in inputs {
        let result = Solution::min_cost(nums, cost);
        println!("{result:?}");
    }
}

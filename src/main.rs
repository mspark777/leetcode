mod utils;

use std::collections::HashMap;

use utils::Solution;

impl Solution {
    pub fn longest_subsequence(arr: Vec<i32>, difference: i32) -> i32 {
        let mut dp = HashMap::<i32, i32>::with_capacity(arr.len());
        let mut result = 0;

        for &num in arr.iter() {
            let d = num - difference;
            let &before = dp.get(&d).unwrap_or(&0);
            let now = before + 1;
            dp.insert(num, now);

            result = result.max(now);
        }

        return result;
    }
}

fn main() {
    let inputs = [
        (vec![1, 2, 3, 4], 1),
        (vec![1, 3, 5, 7], 1),
        (vec![1, 5, 7, 8, 5, 3, 4, 2, 1], -2),
    ];

    for (arr, difference) in inputs {
        let result = Solution::longest_subsequence(arr, difference);
        println!("{result}");
    }
}

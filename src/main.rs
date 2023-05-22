mod utils;

use std::collections::HashMap;

use utils::Solution;

impl Solution {
    pub fn top_k_frequent(nums: Vec<i32>, k: i32) -> Vec<i32> {
        let k = k as usize;
        let mut counts = HashMap::<i32, i32>::with_capacity(k);
        for &num in nums.iter() {
            *counts.entry(num).or_insert(0) += 1;
        }

        let mut frequents = Vec::<(i32, i32)>::with_capacity(counts.len());
        for (&k, &v) in counts.iter() {
            frequents.push((k, v));
        }

        frequents.sort_unstable_by_key(|f| -f.1);
        return frequents.iter().take(k).map(|f| f.0).collect();
    }
}

fn main() {
    let inputs = [(vec![1, 1, 1, 2, 2, 3], 2), (vec![1], 1)];

    for (nums, k) in inputs {
        let result = Solution::top_k_frequent(nums, k);
        println!("{result:?}");
    }
}

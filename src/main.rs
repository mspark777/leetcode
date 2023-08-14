mod utils;

use std::collections::BinaryHeap;

use utils::Solution;

impl Solution {
    pub fn find_kth_largest(nums: Vec<i32>, k: i32) -> i32 {
        let k = k as usize;
        let mut queue = BinaryHeap::<i32>::with_capacity(k + 1);

        for &num in nums.iter() {
            queue.push(-num);
            if queue.len() > k {
                queue.pop();
            }
        }

        let kth = queue.pop().unwrap();
        return -kth;
    }
}

fn main() {
    let inputs = [
        (vec![3, 2, 1, 5, 6, 4], 2),
        (vec![3, 2, 3, 1, 2, 4, 5, 5, 6], 4),
    ];

    for (nums, k) in inputs {
        let result = Solution::find_kth_largest(nums, k);
        println!("{result}");
    }
}

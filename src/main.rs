mod utils;

use std::collections::BinaryHeap;

use utils::Solution;

impl Solution {
    pub fn last_stone_weight(stones: Vec<i32>) -> i32 {
        let mut queue = BinaryHeap::<i32>::with_capacity(stones.len());

        for &s in stones.iter() {
            queue.push(s);
        }

        while let Some(y) = queue.pop() {
            if let Some(x) = queue.pop() {
                queue.push(y - x);
            } else {
                return y;
            }
        }

        return 0;
    }
}

fn main() {
    let inputs = [vec![2, 7, 4, 1, 8, 1], vec![1]];

    for stones in inputs {
        let result = Solution::last_stone_weight(stones);
        println!("{result}");
    }
}

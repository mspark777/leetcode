use std::collections::{HashMap, HashSet};

struct Solution {}
impl Solution {
    pub fn unique_occurrences(arr: Vec<i32>) -> bool {
        let mut counts = HashMap::<i32, i32>::with_capacity(arr.len());
        for n in arr.iter().cloned() {
            *counts.entry(n).or_insert(0) += 1;
        }

        let occurrences: HashSet<i32> = counts.values().cloned().collect();
        return occurrences.len() == counts.len();
    }
}

fn main() {
    let inputs = [
        vec![1, 2, 2, 1, 1, 3],
        vec![1, 2],
        vec![-3, 0, 1, -3, 1, 1, 1, -3, 10, 0],
    ];

    for arr in inputs {
        let result = Solution::unique_occurrences(arr);
        println!("{result}");
    }
}

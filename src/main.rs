/*
leetcode
 */

use std::collections::HashMap;

struct Solution {}
impl Solution {
    pub fn total_fruit(fruits: Vec<i32>) -> i32 {
        let mut baskets = HashMap::<i32, i32>::new();
        let mut left = 0usize;
        let mut result = 0usize;

        for (right, &rfruit) in fruits.iter().enumerate() {
            *baskets.entry(rfruit).or_insert(0) += 1;

            while baskets.len() > 2 {
                let lfruit = fruits[left];
                let lcnt = baskets.get_mut(&lfruit).unwrap();
                if *lcnt > 1 {
                    *lcnt -= 1;
                } else {
                    baskets.remove(&lfruit);
                }

                left += 1;
            }

            result = result.max(right - left + 1);
        }

        return result as i32;
    }
}

fn main() {
    let inputs: Vec<Vec<i32>> = vec![
        vec![1, 2, 1],
        vec![0, 1, 2, 2],
        vec![1, 2, 3, 2, 2],
        vec![3, 3, 3, 1, 2, 1, 1, 2, 3, 3, 4],
    ];

    for input in inputs {
        let result = Solution::total_fruit(input);
        println!("{result}");
    }
}

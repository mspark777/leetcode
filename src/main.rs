use std::collections::HashMap;

struct Solution {}
impl Solution {
    pub fn intersect(nums1: Vec<i32>, nums2: Vec<i32>) -> Vec<i32> {
        let mut counts = HashMap::<i32, i32>::new();
        for &num in nums1.iter() {
            *counts.entry(num).or_insert(0) += 1;
        }

        let mut result = Vec::<i32>::new();
        for num in nums2.iter() {
            if let Some(count) = counts.get_mut(num) {
                if *count > 0 {
                    *count -= 1;
                    result.push(*num);
                }
            }
        }
        return result;
    }
}

fn main() {
    let inputs = [
        vec![vec![1, 2, 2, 1], vec![2, 2]],
        vec![vec![4, 9, 5], vec![9, 4, 9, 8, 4]],
    ];

    for input in inputs {
        let result = Solution::intersect(input[0].clone(), input[1].clone());
        println!("{result:?}");
    }
}

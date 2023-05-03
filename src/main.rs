mod utils;

use std::collections::HashSet;
use std::iter::FromIterator;
use utils::Solution;

impl Solution {
    pub fn find_difference(nums1: Vec<i32>, nums2: Vec<i32>) -> Vec<Vec<i32>> {
        return vec![Self::filter(&nums1, &nums2), Self::filter(&nums2, &nums1)];
    }

    fn filter(nums1: &Vec<i32>, nums2: &Vec<i32>) -> Vec<i32> {
        let set1 = HashSet::<i32>::from_iter(nums2.iter().cloned());
        let set2 = HashSet::<i32>::from_iter(nums1.iter().filter(|n| !set1.contains(n)).cloned());

        return set2.iter().cloned().collect();
    }
}

fn main() {
    let inputs = [
        (vec![1, 2, 3], vec![2, 4, 6]),
        (vec![1, 2, 3, 3], vec![1, 1, 2, 2]),
    ];

    for (nums1, nums2) in inputs {
        let result = Solution::find_difference(nums1, nums2);
        println!("{result:?}");
    }
}

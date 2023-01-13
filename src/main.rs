use std::collections::HashMap;

struct Solution {}
impl Solution {
    pub fn intersection(nums1: Vec<i32>, nums2: Vec<i32>) -> Vec<i32> {
        let mut filter1 = HashMap::<i32, bool>::new();
        for &num in nums1.iter() {
            filter1.insert(num, true);
        }

        let mut result = Vec::<i32>::new();
        for num in nums2.iter() {
            if let Some(ok) = filter1.get_mut(num) {
                if *ok {
                    *ok = false;
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
        let result = Solution::intersection(input[0].clone(), input[1].clone());
        println!("{result:?}");
    }
}

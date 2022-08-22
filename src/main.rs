use std::collections::HashMap;

struct Solution {}
impl Solution {
    pub fn contains_nearby_duplicate(nums: Vec<i32>, k: i32) -> bool {
        let k = k as usize;
        let mut index_map = HashMap::<i32, usize>::with_capacity(nums.len());

        for (i, num) in nums.iter().enumerate() {
            if let Some(idx) = index_map.get(num) {
                if (i - idx) <= k {
                    return true;
                }
            }
            index_map.insert(*num, i);
        }

        false
    }
}

struct Input {
    nums: Vec<i32>,
    k: i32,
}

fn main() {
    let inputs: Vec<Input> = vec![
        Input {
            nums: vec![1, 2, 3, 1],
            k: 3,
        },
        Input {
            nums: vec![1, 0, 1, 1],
            k: 1,
        },
        Input {
            nums: vec![1, 2, 3, 1, 2, 3],
            k: 2,
        },
    ];

    for input in inputs {
        let nums = input.nums;
        let k = input.k;
        let result = Solution::contains_nearby_duplicate(nums, k);
        println!("{:?}", result);
    }
}

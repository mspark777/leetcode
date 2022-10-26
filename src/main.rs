use std::collections::HashMap;

struct Solution {}
impl Solution {
    pub fn check_subarray_sum(nums: Vec<i32>, k: i32) -> bool {
        let mut map = HashMap::<i32, usize>::with_capacity(nums.len());
        let mut sum = 0;

        map.insert(0, 0);

        for i in 0..nums.len() {
            sum += nums[i];
            let m = sum % k;
            if let Some(memo) = map.get(&m) {
                if *memo < i {
                    return true;
                }
            } else {
                map.insert(m, i + 1);
            }
        }
        map.insert(0, 0);

        return false;
    }
}

struct Input {
    nums: Vec<i32>,
    k: i32,
}

fn main() {
    let inputs = [
        Input {
            nums: vec![23, 2, 4, 6, 7],
            k: 6,
        },
        Input {
            nums: vec![23, 2, 6, 4, 7],
            k: 6,
        },
        Input {
            nums: vec![23, 2, 6, 4, 7],
            k: 13,
        },
        Input {
            nums: vec![23, 2, 4, 6, 6],
            k: 7,
        },
    ];

    for Input { nums, k } in inputs {
        let result = Solution::check_subarray_sum(nums, k);
        println!("{result:?}");
    }
}

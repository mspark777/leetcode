use std::collections::HashMap;

struct Solution {}
impl Solution {
    pub fn is_possible(nums: Vec<i32>) -> bool {
        let mut counts = HashMap::<i32, i32>::with_capacity(nums.len());

        for num in nums.iter() {
            let count = counts.entry(*num).or_default();
            *count += 1;
        }

        for num in nums.iter() {
            let mut left = if let Some(cnt) = counts.get(&num) {
                *cnt
            } else {
                0
            };

            if left == 0 {
                continue;
            }

            let mut count = 0;
            let mut cur = *num;
            while let Some(cnt) = counts.get(&cur) {
                if *cnt < left {
                    break;
                }

                left = *cnt;
                *counts.entry(cur).or_default() -= 1;
                count += 1;
                cur += 1;
            }

            if count < 3 {
                return false;
            }
        }

        true
    }
}

struct Input {
    nums: Vec<i32>,
}

fn main() {
    let inputs: Vec<Input> = vec![
        Input {
            nums: vec![1, 2, 3, 3, 4, 5],
        },
        Input {
            nums: vec![1, 2, 3, 3, 4, 4, 5, 5],
        },
        Input {
            nums: vec![1, 2, 3, 4, 4, 5],
        },
    ];

    for input in inputs {
        let nums = input.nums;
        let result = Solution::is_possible(nums);
        println!("{:?}", result);
    }
}

use std::collections::HashMap;

struct Solution {}

impl Solution {
    pub fn maximum_subarray_sum(nums: Vec<i32>, k: i32) -> i64 {
        let mut result = 0i64;
        let mut current_sum = 0i64;
        let mut begin = 0;
        let mut end = 0;
        let mut num_to_index = HashMap::<i32, i32>::with_capacity(nums.len());

        while (end as usize) < nums.len() {
            let current_num = nums[end as usize];
            let last_occurrence = if let Some(&count) = num_to_index.get(&current_num) {
                count
            } else {
                -1
            };

            while begin <= last_occurrence || (end - begin + 1) > k {
                current_sum -= nums[begin as usize] as i64;
                begin += 1;
            }

            num_to_index.insert(current_num, end);
            current_sum += nums[end as usize] as i64;
            if (end - begin + 1) == k {
                result = result.max(current_sum);
            }

            end += 1;
        }

        return result;
    }
}

struct Input {
    nums: Vec<i32>,
    k: i32,
}

fn main() {
    let inputs = vec![
        Input {
            nums: vec![1, 5, 4, 2, 9, 9, 9],
            k: 3,
        },
        Input {
            nums: vec![4, 4, 4],
            k: 3,
        },
    ];

    for input in inputs {
        let result = Solution::maximum_subarray_sum(input.nums, input.k);
        println!("{result}");
    }
}

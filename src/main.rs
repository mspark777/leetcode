struct Solution;

impl Solution {
    pub fn find_missing_elements(nums: Vec<i32>) -> Vec<i32> {
        use std::collections::HashSet;

        let mut nums = nums;
        let n = nums.len();
        nums.sort();

        let range = nums[0]..nums[n - 1];
        let num_set = HashSet::<i32>::from_iter(nums);
        let missing_count = range.len() + 1 - num_set.len();
        if missing_count == 0 {
            return vec![];
        }

        let mut result = Vec::<i32>::with_capacity(missing_count);
        for i in range {
            if !num_set.contains(&i) {
                result.push(i);
            }
        }

        result
    }
}

struct Input {
    nums: Vec<i32>,
}

fn main() {
    let inputs = [
        Input {
            nums: [1, 4, 2, 5].to_vec(),
        },
        Input {
            nums: [7, 8, 6, 9].to_vec(),
        },
        Input {
            nums: [5, 1].to_vec(),
        },
    ];

    for input in inputs {
        let result = Solution::find_missing_elements(input.nums);
        println!("{:?}", result);
    }
}

struct Solution;

impl Solution {
    pub fn smallest_absent(nums: Vec<i32>) -> i32 {
        use std::collections::HashSet;

        let mut sum = 0;
        let n = nums.len() as i32;
        for num in nums.iter().copied() {
            sum += num;
        }

        let n_set = HashSet::<i32>::from_iter(nums);
        let mut result = 0.max(sum / n) + 1;

        while n_set.contains(&result) {
            result += 1;
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
            nums: [3, 5].to_vec(),
        },
        Input {
            nums: [-1, 1, 2].to_vec(),
        },
        Input {
            nums: [4, -1].to_vec(),
        },
    ];

    for input in inputs {
        let result = Solution::smallest_absent(input.nums);
        println!("{:?}", result);
    }
}

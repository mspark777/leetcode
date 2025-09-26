struct Solution {}

impl Solution {
    pub fn target_indices(nums: Vec<i32>, target: i32) -> Vec<i32> {
        let mut count = 0usize;
        let mut less_than = 0;

        for n in nums.iter().cloned() {
            if n == target {
                count += 1;
            } else if n < target {
                less_than += 1;
            }
        }

        let mut result = vec![0; count];
        for i in 0..count {
            result[i] = less_than;
            less_than += 1;
        }

        result
    }
}

struct Input {
    nums: Vec<i32>,
    target: i32,
}

fn main() {
    let inputs = [
        Input {
            nums: [1, 2, 5, 2, 3].to_vec(),
            target: 2,
        },
        Input {
            nums: [1, 2, 5, 2, 3].to_vec(),
            target: 3,
        },
        Input {
            nums: [1, 2, 5, 2, 3].to_vec(),
            target: 5,
        },
    ];

    for input in inputs {
        let result = Solution::target_indices(input.nums, input.target);
        println!("{:?}", result);
    }
}

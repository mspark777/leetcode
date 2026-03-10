struct Solution;

impl Solution {
    pub fn max_sum(nums: Vec<i32>) -> i32 {
        use std::collections::HashSet;
        let set = nums
            .iter()
            .copied()
            .filter(|&n| n > 0)
            .collect::<HashSet<i32>>();

        match set.is_empty() {
            true => nums.iter().copied().max().unwrap(),
            _ => set.iter().sum(),
        }
    }
}

struct Input {
    nums: Vec<i32>,
}

fn main() {
    let inputs = [
        Input {
            nums: [1, 2, 3, 4, 5].to_vec(),
        },
        Input {
            nums: [1, 1, 0, 1, 1].to_vec(),
        },
        Input {
            nums: [1, 2, -1, -2, 1, 0, -1].to_vec(),
        },
    ];

    for input in inputs {
        let result = Solution::max_sum(input.nums);
        println!("{:?}", result);
    }
}

struct Solution {}

impl Solution {
    pub fn minimum_operations(nums: Vec<i32>) -> i32 {
        let s = std::collections::HashSet::<i32>::from_iter(nums.iter().cloned());

        let result = match s.contains(&0) {
            true => s.len() - 1,
            false => s.len(),
        };
        result as i32
    }
}

struct Input {
    nums: Vec<i32>,
}

fn main() {
    let inputs = [
        Input {
            nums: [1, 5, 0, 3, 5].to_vec(),
        },
        Input { nums: [0].to_vec() },
    ];

    for input in inputs {
        let result = Solution::minimum_operations(input.nums);
        println!("{:?}", result);
    }
}

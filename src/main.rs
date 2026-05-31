struct Solution;

impl Solution {
    pub fn next_greater_elements(nums: Vec<i32>) -> Vec<i32> {
        let n = nums.len();
        let mut stack = Vec::<usize>::new();
        let mut result = vec![-1; n];

        for i in 0..(2 * n - 1) {
            while !stack.is_empty() && (nums[*stack.last().unwrap()] < nums[i % n]) {
                result[*stack.last().unwrap()] = nums[i % n];
                stack.pop();
            }
            stack.push(i % n);
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
            nums: [1, 2, 1].to_vec(),
        },
        Input {
            nums: [1, 2, 3, 4, 3].to_vec(),
        },
    ];

    for input in inputs.into_iter() {
        let result = Solution::next_greater_elements(input.nums);
        println!("{:?}", result);
    }
}

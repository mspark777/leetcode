mod utils;

use utils::Solution;

impl Solution {
    pub fn validate_stack_sequences(pushed: Vec<i32>, popped: Vec<i32>) -> bool {
        let num_count = pushed.len();
        let mut stack = Vec::<i32>::with_capacity(num_count);
        let mut pop_count = 0usize;

        for &p in pushed.iter() {
            stack.push(p);
            while let Some(&top) = stack.last() {
                if pop_count >= num_count {
                    break;
                } else if top != popped[pop_count] {
                    break;
                } else {
                    stack.pop();
                    pop_count += 1;
                }
            }
        }

        return num_count == pop_count;
    }
}

fn main() {
    let inputs = [
        (vec![1, 2, 3, 4, 5], vec![4, 5, 3, 2, 1]),
        (vec![1, 2, 3, 4, 5], vec![4, 3, 5, 1, 2]),
    ];

    for (pushed, popped) in inputs {
        let result = Solution::validate_stack_sequences(pushed, popped);
        println!("{result}");
    }
}

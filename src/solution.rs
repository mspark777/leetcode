use std::collections::BinaryHeap;

pub struct Solution {}

impl Solution {
    pub fn is_possible(target: Vec<i32>) -> bool {
        let mut queue = BinaryHeap::<i32>::with_capacity(target.len());

        let mut sum = target.iter().fold(0, |acc, cur| {
            queue.push(*cur);
            acc + *cur
        });

        while let Some(top) = queue.pop() {
            if top == 1 {
                break;
            }

            sum -= top;
            if (top <= sum) || (sum < 1) {
                return false;
            }

            let top = top % sum;
            sum += top;
            if top > 0 {
                queue.push(top);
            } else {
                queue.push(sum);
            }
        }

        true
    }
}

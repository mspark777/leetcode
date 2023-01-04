use std::collections::HashMap;

struct Solution {}
impl Solution {
    pub fn minimum_rounds(tasks: Vec<i32>) -> i32 {
        let mut counts = HashMap::<i32, i32>::new();
        for &task in tasks.iter() {
            *counts.entry(task).or_insert(0) += 1;
        }

        let mut result = 0;

        for &count in counts.values() {
            if count == 1 {
                return -1;
            }

            result += count / 3;
            if (count % 3) != 0 {
                result += 1;
            }
        }

        return result;
    }
}

fn main() {
    let inputs = [vec![2, 2, 3, 3, 2, 4, 4, 4, 4, 4], vec![2, 3, 3]];

    for tasks in inputs {
        let result = Solution::minimum_rounds(tasks);
        println!("{result}");
    }
}

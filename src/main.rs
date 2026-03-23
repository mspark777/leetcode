struct Solution;

impl Solution {
    pub fn earliest_time(tasks: Vec<Vec<i32>>) -> i32 {
        tasks
            .into_iter()
            .map(|v| v.into_iter().sum())
            .min()
            .unwrap_or_default()
    }
}

struct Input {
    tasks: Vec<Vec<i32>>,
}

fn main() {
    let inputs = [
        Input {
            tasks: [[1, 6], [2, 3]].map(|v| v.to_vec()).to_vec(),
        },
        Input {
            tasks: [[100, 100], [100, 100]].map(|v| v.to_vec()).to_vec(),
        },
    ];

    for input in inputs {
        let result = Solution::earliest_time(input.tasks);
        println!("{:?}", result);
    }
}

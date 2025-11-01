struct Solution {}

impl Solution {
    pub fn hardest_worker(_n: i32, logs: Vec<Vec<i32>>) -> i32 {
        let mut prev_leave_time = 0;
        let mut longest_time = 0;
        let mut result = 0;
        for log in logs.iter() {
            let id = log[0];
            let leave_time = log[1];

            let time = leave_time - prev_leave_time;
            prev_leave_time = leave_time;
            if longest_time < time {
                longest_time = time;
                result = id;
            } else if longest_time == time {
                if result > id {
                    result = id;
                }
            }
        }

        result
    }
}

struct Input {
    n: i32,
    logs: Vec<Vec<i32>>,
}

fn main() {
    let inputs = [
        Input {
            n: 10,
            logs: [[0, 3], [2, 5], [0, 9], [1, 15]]
                .map(|a| a.to_vec())
                .to_vec(),
        },
        Input {
            n: 26,
            logs: [[1, 1], [3, 7], [2, 12], [7, 17]]
                .map(|a| a.to_vec())
                .to_vec(),
        },
        Input {
            n: 2,
            logs: [[0, 10], [1, 20]].map(|a| a.to_vec()).to_vec(),
        },
    ];

    for input in inputs {
        let result = Solution::hardest_worker(input.n, input.logs);
        println!("{:?}", result);
    }
}

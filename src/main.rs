struct Solution {}

impl Solution {
    pub fn max_free_time(event_time: i32, start_time: Vec<i32>, end_time: Vec<i32>) -> i32 {
        let n = start_time.len();
        let mut q = vec![false; n];
        let mut t1 = 0;
        let mut t2 = 0;

        for i in 0..n {
            if end_time[i] - start_time[i] <= t1 {
                q[i] = true;
            }

            if i == 0 {
                t1 = t1.max(start_time[i]);
            } else {
                t1 = t1.max(start_time[i] - end_time[i - 1]);
            }

            let j = n - (i + 1);
            if end_time[j] - start_time[j] <= t2 {
                q[j] = true;
            }

            if i == 0 {
                t2 = t2.max(event_time - end_time[j]);
            } else {
                t2 = t2.max(start_time[n - i] - end_time[j]);
            }
        }

        let mut result = 0;
        for i in 0..n {
            let left = if i == 0 { 0 } else { end_time[i - 1] };
            let right = if i == n - 1 {
                event_time
            } else {
                start_time[i + 1]
            };

            result = if q[i] {
                result.max(right - left)
            } else {
                result.max(right - left - (end_time[i] - start_time[i]))
            }
        }

        result
    }
}

struct Input {
    event_time: i32,
    start_time: Vec<i32>,
    end_time: Vec<i32>,
}

fn main() {
    let inputs = vec![
        Input {
            event_time: 5,
            start_time: [1, 3].to_vec(),
            end_time: [2, 5].to_vec(),
        },
        Input {
            event_time: 10,
            start_time: [0, 7, 9].to_vec(),
            end_time: [1, 8, 10].to_vec(),
        },
        Input {
            event_time: 10,
            start_time: [0, 3, 7, 9].to_vec(),
            end_time: [1, 4, 8, 10].to_vec(),
        },
        Input {
            event_time: 5,
            start_time: [0, 1, 2, 3, 4].to_vec(),
            end_time: [1, 2, 3, 4, 5].to_vec(),
        },
        Input {
            event_time: 17,
            start_time: [6, 9, 16].to_vec(),
            end_time: [8, 16, 17].to_vec(),
        },
    ];

    for input in inputs {
        let result = Solution::max_free_time(input.event_time, input.start_time, input.end_time);
        println!("{:?}", result);
    }
}

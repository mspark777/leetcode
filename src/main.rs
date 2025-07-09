struct Solution {}

impl Solution {
    pub fn max_free_time(event_time: i32, k: i32, start_time: Vec<i32>, end_time: Vec<i32>) -> i32 {
        let n = start_time.len();
        let k = k as usize;
        let mut result = 0;
        let mut t = 0;

        for i in 0..n {
            t += end_time[i] - start_time[i];
            let left = if i <= k - 1 { 0 } else { end_time[i - k] };
            let right = if i == n - 1 {
                event_time
            } else {
                start_time[i + 1]
            };

            result = result.max(right - left - t);
            if i >= k - 1 {
                let j = (i + 1) - k;
                t -= end_time[j] - start_time[j];
            }
        }

        result
    }
}

struct Input {
    event_time: i32,
    k: i32,
    start_time: Vec<i32>,
    end_time: Vec<i32>,
}

fn main() {
    let inputs = vec![
        Input {
            event_time: 5,
            k: 1,
            start_time: [1, 3].to_vec(),
            end_time: [2, 4].to_vec(),
        },
        Input {
            event_time: 10,
            k: 1,
            start_time: [0, 2, 9].to_vec(),
            end_time: [1, 4, 10].to_vec(),
        },
        Input {
            event_time: 5,
            k: 2,
            start_time: [0, 1, 2, 3, 4].to_vec(),
            end_time: [1, 2, 3, 4, 5].to_vec(),
        },
    ];

    for input in inputs {
        let result =
            Solution::max_free_time(input.event_time, input.k, input.start_time, input.end_time);
        println!("{:?}", result);
    }
}

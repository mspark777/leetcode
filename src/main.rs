struct Solution {}

impl Solution {
    pub fn max_value(events: Vec<Vec<i32>>, k: i32) -> i32 {
        let mut events = events;
        let k = k as usize;
        let n = events.len();
        let mut dp = vec![vec![0; n + 1]; k + 1];

        events.sort_unstable_by_key(|e| e[0]);

        for cur in (0..n).rev() {
            let next = Self::bsearch(&events, events[cur][1]);
            for count in 1..=k {
                dp[count][cur] = dp[count][cur + 1].max(events[cur][2] + dp[count - 1][next]);
            }
        }

        dp[k][0]
    }

    fn bsearch(events: &Vec<Vec<i32>>, target: i32) -> usize {
        let mut left = 0;
        let mut right = events.len();
        while left < right {
            let mid = (left + right) / 2;
            if events[mid][0] <= target {
                left = mid + 1;
            } else {
                right = mid;
            }
        }

        left
    }
}

struct Input {
    events: Vec<Vec<i32>>,
    k: i32,
}

fn main() {
    let inputs = vec![
        Input {
            events: [[1, 2, 4], [3, 4, 3], [2, 3, 1]]
                .iter()
                .map(|v| v.to_vec())
                .collect(),
            k: 2,
        },
        Input {
            events: [[1, 2, 4], [3, 4, 3], [2, 3, 10]]
                .iter()
                .map(|v| v.to_vec())
                .collect(),
            k: 2,
        },
        Input {
            events: [[1, 1, 1], [2, 2, 2], [3, 3, 3], [4, 4, 4]]
                .iter()
                .map(|v| v.to_vec())
                .collect(),
            k: 3,
        },
    ];

    for input in inputs {
        let result = Solution::max_value(input.events, input.k);
        println!("{:?}", result);
    }
}

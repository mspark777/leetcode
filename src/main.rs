mod utils;

use utils::Solution;

impl Solution {
    pub fn merge(intervals: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        let mut intervals = intervals;
        intervals.sort_unstable_by_key(|i| i[0]);

        let mut result = vec![intervals[0].clone()];
        for interval in intervals.iter().skip(1) {
            let last = result.last_mut().unwrap();
            let last_end = last[1];
            let start = interval[0];
            if last_end < start {
                result.push(interval.clone());
            } else {
                let end = interval[1];
                last[1] = last_end.max(end);
            }
        }

        return result;
    }
}

fn main() {
    let inputs = [
        vec![[1, 3], [2, 6], [8, 10], [15, 18]],
        vec![[1, 4], [4, 5]],
    ];

    for intervals in inputs {
        let intervals = intervals.iter().map(|i| i.to_vec()).collect();
        let result = Solution::merge(intervals);
        println!("{result:?}");
    }
}

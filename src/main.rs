mod utils;

use utils::Solution;

impl Solution {
    pub fn erase_overlap_intervals(intervals: Vec<Vec<i32>>) -> i32 {
        let mut intervals = intervals;
        intervals.sort_unstable_by_key(|i| i[1]);

        let mut result = 0;
        let mut k = i32::MIN;
        for interval in intervals.iter() {
            let x = interval[0];
            let y = interval[1];

            if x >= k {
                k = y;
            } else {
                result += 1;
            }
        }

        return result;
    }
}

fn main() {
    let inputs = [
        vec![vec![1, 2], vec![2, 3], vec![3, 4], vec![1, 3]],
        vec![vec![1, 2], vec![1, 2], vec![1, 2]],
        vec![vec![1, 2], vec![2, 3]],
    ];

    for intervals in inputs {
        let result = Solution::erase_overlap_intervals(intervals);
        println!("{result}");
    }
}

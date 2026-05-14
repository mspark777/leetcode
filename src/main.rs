struct Solution;

impl Solution {
    pub fn find_right_interval(intervals: Vec<Vec<i32>>) -> Vec<i32> {
        use std::collections::BTreeMap;

        let mut btree = BTreeMap::<i32, usize>::new();
        for (i, interval) in intervals.iter().enumerate() {
            let start = interval[0];
            btree.insert(start, i);
        }

        let mut result = Vec::<i32>::with_capacity(intervals.len());
        for interval in intervals.iter() {
            let end = interval[1];
            if let Some(node) = btree.range(end..).next() {
                let idx = *node.1 as i32;
                result.push(idx);
            } else {
                result.push(-1);
            }
        }
        result
    }
}

struct Input {
    intervals: Vec<Vec<i32>>,
}

fn main() {
    let inputs = [
        Input {
            intervals: [[1, 2]].map(|v| v.to_vec()).to_vec(),
        },
        Input {
            intervals: [[3, 4], [2, 3], [1, 2]].map(|v| v.to_vec()).to_vec(),
        },
        Input {
            intervals: [[1, 4], [2, 3], [3, 4]].map(|v| v.to_vec()).to_vec(),
        },
    ];

    for input in inputs.into_iter() {
        let result = Solution::find_right_interval(input.intervals);
        println!("{:?}", result);
    }
}

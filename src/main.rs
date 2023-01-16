struct Solution {}
impl Solution {
    pub fn insert(intervals: Vec<Vec<i32>>, new_interval: Vec<i32>) -> Vec<Vec<i32>> {
        let mut intervals = intervals;
        let pos = intervals.binary_search_by(|i| i[0].cmp(&new_interval[0]));

        match pos {
            Ok(p) => intervals.insert(p, new_interval),
            Err(p) => {
                if p < intervals.len() {
                    intervals.insert(p, new_interval);
                } else {
                    intervals.push(new_interval);
                }
            }
        }

        let mut result = vec![intervals[0].clone()];

        for i in 1..intervals.len() {
            let last = result.last_mut().unwrap();
            let interval = &intervals[i];
            if interval[0] > last[1] {
                result.push(interval.clone());
            } else {
                last[1] = last[1].max(interval[1]);
            }
        }

        return result;
    }
}

struct Input {
    intervals: Vec<Vec<i32>>,
    new_interval: Vec<i32>,
}

fn main() {
    let inputs = [
        Input {
            intervals: vec![vec![1, 3], vec![6, 9]],
            new_interval: vec![2, 5],
        },
        Input {
            intervals: vec![
                vec![1, 2],
                vec![3, 5],
                vec![6, 7],
                vec![8, 10],
                vec![12, 16],
            ],
            new_interval: vec![4, 8],
        },
        Input {
            intervals: vec![],
            new_interval: vec![5, 7],
        },
    ];

    for Input {
        intervals,
        new_interval,
    } in inputs
    {
        let result = Solution::insert(intervals, new_interval);
        println!("{result:?}");
    }
}

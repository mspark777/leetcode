use std::cmp::Reverse;

struct Solution {}

impl Solution {
    pub fn max_two_events(events: Vec<Vec<i32>>) -> i32 {
        return Self::solve(events.clone());
    }

    fn solve(mut events: Vec<Vec<i32>>) -> i32 {
        events.sort_unstable_by_key(|e| (e[0], e[1]));

        let mut queue =
            std::collections::binary_heap::BinaryHeap::<Reverse<(i32, i32)>>::with_capacity(
                events.len(),
            );

        let mut result = 0;
        let mut max_val = 0;
        for event in events.iter() {
            let start = event[0];
            let end = event[1];
            let val = event[2];
            while let Some(&Reverse((s, e))) = queue.peek() {
                if s < start {
                    max_val = max_val.max(e);
                    queue.pop();
                } else {
                    break;
                }
            }

            result = result.max(max_val + val);
            queue.push(Reverse((end, val)));
        }

        return result;
    }
}

struct Input {
    events: Vec<Vec<i32>>,
}

fn main() {
    let inputs = vec![
        Input {
            events: vec![vec![1, 3, 2], vec![4, 5, 2], vec![2, 4, 3]],
        },
        Input {
            events: vec![vec![1, 3, 2], vec![4, 5, 2], vec![1, 5, 5]],
        },
        Input {
            events: vec![vec![1, 5, 3], vec![1, 5, 1], vec![6, 6, 5]],
        },
    ];

    for input in inputs {
        let result = Solution::max_two_events(input.events);
        println!("{result}");
    }
}

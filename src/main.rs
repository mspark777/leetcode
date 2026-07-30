struct Solution;

use std::cmp::Reverse;
use std::collections::BinaryHeap;

impl Solution {
    pub fn network_delay_time(times: Vec<Vec<i32>>, n: i32, k: i32) -> i32 {
        let n = n as usize;
        let k = k as usize;
        let mut graph = vec![Vec::<(usize, i32)>::new(); n + 1];
        for time in times {
            let u = time[0];
            let v = time[1];
            let w = time[2];
            graph[u as usize].push((v as usize, time[2]));
        }

        let mut queue = BinaryHeap::<Reverse<(i32, usize)>>::new();
        let mut delays = vec![None as Option<i32>; n + 1];
        let mut visited = 0;

        queue.push(Reverse((0, k)));

        while let Some(Reverse((delay, node))) = queue.pop() {
            match delays[node] {
                None => {
                    delays[node] = Some(delay);
                    visited += 1;
                    if visited == n {
                        break;
                    }
                }

                Some(d) if d <= delay => {
                    continue;
                }
                _ => {
                    delays[node] = Some(delay);
                }
            }

            for (edge, weight) in graph[node].iter().copied() {
                if delays[edge].is_none() || delays[edge] > Some(delay + weight) {
                    queue.push(Reverse((delay + weight, edge)));
                }
            }
        }

        if visited != n {
            return -1;
        }

        delays
            .into_iter()
            .skip(1)
            .map(|x| x.unwrap_or_default())
            .max()
            .unwrap_or_default()
    }
}

struct Input {
    nums: Vec<i32>,
}

fn main() {
    let inputs = [
        Input {
            nums: vec![3, 4, 2],
        },
        Input {
            nums: vec![2, 2, 3, 3, 3, 4],
        },
    ];

    for input in inputs.into_iter() {
        let result = Solution::delete_and_earn(input.nums);
        println!("{:?}", result);
    }
}

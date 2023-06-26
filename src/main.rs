mod utils;

use std::{cmp::Ordering, collections::BinaryHeap};

use utils::Solution;

#[derive(Eq, PartialEq)]
struct Node {
    cost: i32,
    section: bool,
}

impl Ord for Node {
    fn cmp(&self, other: &Self) -> Ordering {
        let diff = self.cost - other.cost;
        if diff < 0 {
            return Ordering::Greater;
        } else if diff > 0 {
            return Ordering::Less;
        }

        return if self.section {
            Ordering::Less
        } else {
            Ordering::Greater
        };
    }
}

impl PartialOrd for Node {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        return Some(self.cmp(other));
    }
}

impl Solution {
    pub fn total_cost(costs: Vec<i32>, k: i32, candidates: i32) -> i64 {
        let k = k as usize;
        let candidates = candidates as usize;
        let mut queue = BinaryHeap::<Node>::with_capacity(costs.len());
        for i in 0..candidates {
            queue.push(Node {
                cost: costs[i],
                section: false,
            });
        }

        for i in candidates.max(costs.len() - candidates)..costs.len() {
            queue.push(Node {
                cost: costs[i],
                section: true,
            });
        }

        let mut result = 0i64;
        let mut head = candidates;

        let tail = costs.len() as i64 - (1 + candidates) as i64;
        let mut tail = tail.max(0) as usize;
        for _i in 0..k {
            let Node { cost, section } = queue.pop().unwrap();
            result += cost as i64;

            if head <= tail {
                if section {
                    queue.push(Node {
                        cost: costs[tail],
                        section: true,
                    });
                    tail -= 1;
                } else {
                    queue.push(Node {
                        cost: costs[head],
                        section: false,
                    });
                    head += 1;
                }
            }
        }

        return result;
    }
}

fn main() {
    let inputs = [
        (vec![17, 12, 10, 2, 7, 2, 11, 20, 8], 3, 4),
        (vec![1, 2, 4, 1], 3, 3),
        (
            vec![
                69, 10, 63, 24, 1, 71, 55, 46, 4, 61, 78, 21, 85, 52, 83, 77, 42, 21, 73, 2, 80,
                99, 98, 89, 55, 94, 63, 50, 43, 62, 14,
            ],
            21,
            31,
        ),
        (vec![10, 1, 11, 10], 2, 1),
    ];

    for (costs, k, candidates) in inputs {
        let result = Solution::total_cost(costs, k, candidates);
        println!("{result:?}");
    }
}

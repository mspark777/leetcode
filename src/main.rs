mod utils;
use std::collections::HashMap;

use utils::Solution;

impl Solution {
    pub fn min_reorder(n: i32, connections: Vec<Vec<i32>>) -> i32 {
        let mut adjs = HashMap::<i32, Vec<(i32, i32)>>::with_capacity(n as usize);
        for conn in connections.iter() {
            let a = conn[0];
            let b = conn[1];
            adjs.entry(a).or_insert(vec![]).push((b, 1));
            adjs.entry(b).or_insert(vec![]).push((a, 0));
        }

        let mut result = 0;
        Self::dfs(0, -1, &adjs, &mut result);
        return result;
    }

    fn dfs(node: i32, parent: i32, adjs: &HashMap<i32, Vec<(i32, i32)>>, count: &mut i32) {
        if let Some(edges) = adjs.get(&node) {
            for &(child, sign) in edges.iter() {
                if child != parent {
                    *count += sign;
                    Self::dfs(child, node, adjs, count);
                }
            }
        }
    }
}

struct Input {
    n: i32,
    connections: Vec<Vec<i32>>,
}

fn main() {
    let inputs = [
        Input {
            n: 4,
            connections: vec![vec![0, 1], vec![1, 3], vec![2, 3], vec![4, 0], vec![4, 5]],
        },
        Input {
            n: 6,
            connections: vec![vec![1, 0], vec![1, 2], vec![3, 2], vec![3, 4]],
        },
        Input {
            n: 6,
            connections: vec![vec![1, 0], vec![2, 0]],
        },
    ];

    for Input { n, connections } in inputs {
        let result = Solution::min_reorder(n, connections);
        println!("{result}")
    }
}

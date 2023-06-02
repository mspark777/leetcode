mod utils;

use std::collections::{HashMap, HashSet};

use utils::Solution;

impl Solution {
    pub fn maximum_detonation(bombs: Vec<Vec<i32>>) -> i32 {
        let mut graph = HashMap::<usize, Vec<usize>>::with_capacity(bombs.len());

        for (i, ibomb) in bombs.iter().enumerate() {
            for (j, jbomb) in bombs.iter().enumerate() {
                if i == j {
                    continue;
                }

                let ix = ibomb[0] as i64;
                let iy = ibomb[1] as i64;
                let ir = ibomb[2] as i64;
                let jx = jbomb[0] as i64;
                let jy = jbomb[1] as i64;
                let dx = ix - jx;
                let dy = iy - jy;
                let d = (dx * dx) + (dy * dy);
                let r = ir * ir;
                if r < d {
                    continue;
                }

                graph
                    .entry(i)
                    .or_insert(Vec::<usize>::with_capacity(bombs.len()))
                    .push(j);
            }
        }

        let mut result = 0;
        for i in 0..bombs.len() {
            result = result.max(Self::dfs(i, &graph));
        }

        return result;
    }

    fn dfs(i: usize, graph: &HashMap<usize, Vec<usize>>) -> i32 {
        let mut stack = Vec::<usize>::with_capacity(graph.len());
        let mut visited = HashSet::<usize>::with_capacity(graph.len());

        stack.push(i);
        visited.insert(i);

        while let Some(top) = stack.pop() {
            if let Some(nodes) = graph.get(&top) {
                for &node in nodes.iter() {
                    if !visited.contains(&node) {
                        stack.push(node);
                        visited.insert(node);
                    }
                }
            }
        }

        return visited.len() as i32;
    }
}

fn main() {
    let inputs = [
        vec![vec![2, 1, 3], vec![6, 1, 4]],
        vec![vec![1, 1, 5], vec![10, 10, 5]],
        vec![
            vec![1, 2, 3],
            vec![2, 3, 1],
            vec![3, 4, 2],
            vec![4, 5, 3],
            vec![5, 6, 4],
        ],
    ];

    for bombs in inputs {
        let result = Solution::maximum_detonation(bombs);
        println!("{result}");
    }
}

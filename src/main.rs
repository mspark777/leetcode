struct Solution;

use std::collections::HashSet;
impl Solution {
    pub fn loud_and_rich(richer: Vec<Vec<i32>>, quiet: Vec<i32>) -> Vec<i32> {
        let mut result: Vec<i32> = vec![];
        let mut graph: Vec<Vec<i32>> = vec![vec![]; quiet.len()];

        for edge in richer {
            graph[edge[1] as usize].push(edge[0])
        }

        for item in 0..quiet.len() {
            let mut stack = vec![item];
            let mut visited = HashSet::<usize>::new();
            let mut louder = item;
            let mut min = quiet[item];

            while let Some(item) = stack.pop() {
                if visited.contains(&item) {
                    continue;
                }

                visited.insert(item);
                if quiet[item] < min {
                    min = quiet[item];
                    louder = item;
                }

                for dest_item in graph[item].iter().copied() {
                    stack.push(dest_item as usize);
                }
            }

            result.push(louder as i32);
        }

        result
    }
}

struct Input {
    richer: Vec<Vec<i32>>,
    quiet: Vec<i32>,
}

fn main() {
    let inputs = [Input {
        richer: [[1, 0], [2, 1], [3, 1], [3, 7], [4, 3], [5, 3], [6, 3]]
            .map(|v| v.to_vec())
            .to_vec(),
        quiet: [3, 2, 5, 4, 6, 1, 7, 0].to_vec(),
    }];

    for input in inputs {
        let result = Solution::loud_and_rich(input.richer, input.quiet);
        println!("{:?}", result);
    }
}

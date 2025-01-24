struct Solution {}

impl Solution {
    pub fn eventual_safe_nodes(graph: Vec<Vec<i32>>) -> Vec<i32> {
        let n = graph.len();
        let mut indegrees = vec![0; n];
        let mut adjacents = vec![Vec::<usize>::new(); n];

        for (i, nodes) in graph.iter().enumerate() {
            for node in nodes.iter().cloned() {
                adjacents[node as usize].push(i);
                indegrees[i] += 1;
            }
        }

        let mut queue = std::collections::VecDeque::with_capacity(n);
        for (i, indegree) in indegrees.iter().cloned().enumerate() {
            if indegree == 0 {
                queue.push_back(i);
            }
        }

        let mut safe = vec![false; n];

        while let Some(node) = queue.pop_front() {
            safe[node] = true;

            for neighbor in adjacents[node].iter().cloned() {
                indegrees[neighbor] -= 1;
                if indegrees[neighbor] == 0 {
                    queue.push_back(neighbor);
                }
            }
        }

        let mut result = Vec::<i32>::with_capacity(n);
        for (i, s) in safe.iter().cloned().enumerate() {
            if s {
                result.push(i as i32);
            }
        }

        return result;
    }
}

struct Input {
    graph: Vec<Vec<i32>>,
}

fn main() {
    let inputs = vec![
        Input {
            graph: vec![
                vec![1, 2],
                vec![2, 3],
                vec![5],
                vec![0],
                vec![5],
                vec![],
                vec![],
            ],
        },
        Input {
            graph: vec![vec![1, 2, 3, 4], vec![1, 2], vec![3, 4], vec![0, 4], vec![]],
        },
    ];

    for input in inputs {
        let result = Solution::eventual_safe_nodes(input.graph);
        println!("{result:?}");
    }
}

struct Solution {}

impl Solution {
    pub fn max_k_divisible_components(
        n: i32,
        edges: Vec<Vec<i32>>,
        values: Vec<i32>,
        k: i32,
    ) -> i32 {
        if n < 2 {
            return 1;
        }

        let mut result = 0;
        let mut graph = vec![vec![]; n as usize];
        let mut in_degree = vec![0; n as usize];

        for edge in edges.iter() {
            let node1 = edge[0] as usize;
            let node2 = edge[1] as usize;
            graph[node1].push(node2);
            graph[node2].push(node1);
            in_degree[node1] += 1;
            in_degree[node2] += 1
        }

        let mut long_values = values.iter().map(|&n| n as usize).collect::<Vec<_>>();
        let mut queue = std::collections::VecDeque::<usize>::with_capacity(in_degree.len());
        for (node, &degree) in in_degree.iter().enumerate() {
            if degree == 1 {
                queue.push_back(node);
            }
        }

        while let Some(current_node) = queue.pop_front() {
            in_degree[current_node] -= 1;
            let mut add_value = 0;
            if long_values[current_node] % (k as usize) == 0 {
                result += 1;
            } else {
                add_value = long_values[current_node];
            }

            for &neighbor_node in graph[current_node].iter() {
                if in_degree[neighbor_node] == 0 {
                    continue;
                }

                in_degree[neighbor_node] -= 1;
                long_values[neighbor_node] += add_value;
                if in_degree[neighbor_node] == 1 {
                    queue.push_back(neighbor_node);
                }
            }
        }

        return result;
    }
}

struct Input {
    n: i32,
    edges: Vec<Vec<i32>>,
    values: Vec<i32>,
    k: i32,
}

fn main() {
    let inputs = vec![
        Input {
            n: 5,
            edges: vec![vec![0, 2], vec![1, 2], vec![1, 3], vec![2, 4]],
            values: vec![1, 8, 1, 4, 4],
            k: 6,
        },
        Input {
            n: 7,
            edges: vec![
                vec![0, 1],
                vec![0, 2],
                vec![1, 3],
                vec![1, 4],
                vec![2, 5],
                vec![2, 6],
            ],
            values: vec![3, 0, 6, 1, 5, 2, 1],
            k: 3,
        },
    ];

    for input in inputs {
        let result =
            Solution::max_k_divisible_components(input.n, input.edges, input.values, input.k);
        println!("{result}");
    }
}

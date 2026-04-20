struct Solution;

impl Solution {
    pub fn find_min_height_trees(n: i32, edges: Vec<Vec<i32>>) -> Vec<i32> {
        use std::collections::VecDeque;

        if n == 1 {
            return vec![0];
        }

        let n = n as usize;
        let mut adjacent_nodes = vec![Vec::<i32>::new(); n];
        let mut degree = vec![0; n];

        for edge in edges.iter() {
            let a = edge[0];
            let b = edge[1];

            adjacent_nodes[a as usize].push(b);
            adjacent_nodes[b as usize].push(a);

            degree[a as usize] += 1;
            degree[b as usize] += 1;
        }

        let mut queue = VecDeque::<i32>::new();
        for (i, d) in degree.iter().copied().enumerate() {
            if d == 1 {
                queue.push_back(i as i32);
            }
        }

        let mut nodes = n;
        while nodes > 2 {
            let size = queue.len();
            nodes -= size;

            for _ in 0..size {
                let node = queue.pop_front().unwrap();
                for adjacent in adjacent_nodes[node as usize].iter().copied() {
                    degree[adjacent as usize] -= 1;

                    if degree[adjacent as usize] == 1 {
                        queue.push_back(adjacent);
                    }
                }
            }
        }

        queue.into_iter().collect()
    }
}

struct Input {
    n: i32,
    edges: Vec<Vec<i32>>,
}

fn main() {
    let inputs = [Input {
        n: 4,
        edges: [[1, 0], [1, 2], [1, 3]].map(|v| v.to_vec()).to_vec(),
    }];

    for input in inputs {
        let result = Solution::find_min_height_trees(input.n, input.edges);
        println!("{:?}", result);
    }
}

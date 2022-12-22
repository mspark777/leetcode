use std::collections::HashSet;

struct DFS {
    result: Vec<i32>,
    counts: Vec<i32>,
    graph: Vec<HashSet<usize>>,
    n: i32,
}

impl DFS {
    fn new(n: usize) -> Self {
        let result = vec![0; n];
        let counts = vec![1; n];
        let graph = vec![HashSet::<usize>::new(); n];

        return Self {
            result,
            counts,
            graph,
            n: n as i32,
        };
    }

    fn dfs(&mut self, node: usize, parent: usize) {
        let graph: Vec<usize> = self.graph[node].iter().cloned().collect();
        for child in graph {
            if child != parent {
                self.dfs(child, node);
                self.counts[node] += self.counts[child];
                self.result[node] += self.result[child] + self.counts[child];
            }
        }
    }

    fn dfs2(&mut self, node: usize, parent: usize) {
        let graph: Vec<usize> = self.graph[node].iter().cloned().collect();
        for child in graph {
            if child != parent {
                self.result[child] =
                    self.result[node] - self.counts[child] + self.n - self.counts[child];
                self.dfs2(child, node);
            }
        }
    }

    fn sum_of_distances_in_tree(&mut self, edges: &Vec<Vec<i32>>) -> Vec<i32> {
        for edge in edges.iter() {
            let i = edge[0] as usize;
            let j = edge[1] as usize;
            self.graph[i].insert(j);
            self.graph[j].insert(i);
        }

        self.dfs(0, usize::max_value());
        self.dfs2(0, usize::max_value());
        return self.result.clone();
    }
}

struct Solution {}
impl Solution {
    pub fn sum_of_distances_in_tree(n: i32, edges: Vec<Vec<i32>>) -> Vec<i32> {
        let mut dfs = DFS::new(n as usize);
        return dfs.sum_of_distances_in_tree(&edges);
    }
}

struct Input {
    n: i32,
    edges: Vec<Vec<i32>>,
}

fn main() {
    let inputs = [
        Input {
            n: 6,
            edges: vec![vec![0, 1], vec![0, 2], vec![2, 3], vec![2, 4], vec![2, 5]],
        },
        Input {
            n: 1,
            edges: vec![],
        },
        Input {
            n: 2,
            edges: vec![vec![1, 0]],
        },
    ];

    for Input { n, edges } in inputs {
        let result = Solution::sum_of_distances_in_tree(n, edges);
        println!("{result:?}");
    }
}

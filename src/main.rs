struct Solution {}

struct Helper {
    parent: Vec<usize>,
    depth: Vec<usize>,
    n: usize,
}

impl Helper {
    fn new(n: usize) -> Self {
        return Self {
            n,
            parent: vec![n; n],
            depth: vec![0; n],
        };
    }

    pub fn find(&mut self, node: usize) -> usize {
        if self.parent[node] == self.n {
            return node;
        }

        self.parent[node] = self.find(self.parent[node]);
        return self.parent[node];
    }

    pub fn union(&mut self, node1: usize, node2: usize) {
        let mut root1 = self.find(node1);
        let mut root2 = self.find(node2);

        if root1 == root2 {
            return;
        }

        if self.depth[root1] < self.depth[root2] {
            let t1 = root1;
            let t2 = root2;
            root1 = t2;
            root2 = t1;
        }

        self.parent[root2] = root1;
        if self.depth[root1] == self.depth[root2] {
            self.depth[root1] += 1;
        }
    }
}

impl Solution {
    pub fn minimum_cost(n: i32, edges: Vec<Vec<i32>>, query: Vec<Vec<i32>>) -> Vec<i32> {
        let n = n as usize;
        let mut helper = Helper::new(n);

        for edge in edges.iter() {
            helper.union(edge[0] as usize, edge[1] as usize);
        }

        let mut component_cost = vec![u32::MAX; n];
        for edge in edges.iter() {
            let root = helper.find(edge[0] as usize);
            component_cost[root] &= edge[2] as u32;
        }

        let mut result = Vec::<i32>::new();
        for q in query.iter() {
            let start = q[0] as usize;
            let end = q[1] as usize;

            if helper.find(start) != helper.find(end) {
                result.push(-1);
            } else {
                let root = helper.find(start);
                result.push(component_cost[root] as i32);
            }
        }
        return result;
    }
}

struct Input {
    n: i32,
    edges: Vec<Vec<i32>>,
    query: Vec<Vec<i32>>,
}

fn main() {
    let inputs = vec![
        Input {
            n: 5,
            edges: vec![vec![0, 1, 7], vec![1, 3, 7], vec![1, 2, 1]],
            query: vec![vec![0, 3], vec![3, 4]],
        },
        Input {
            n: 3,
            edges: vec![vec![0, 2, 7], vec![0, 1, 15], vec![1, 2, 1]],
            query: vec![vec![1, 2]],
        },
    ];

    for input in inputs {
        let result = Solution::minimum_cost(input.n, input.edges, input.query);
        println!("{result:?}");
    }
}

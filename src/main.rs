struct Solution {}

struct DSU {
    size: Vec<usize>,
    representative: Vec<usize>,
}

impl DSU {
    fn new(n: usize) -> Self {
        return Self {
            size: vec![1; n],
            representative: (0..n).collect(),
        };
    }

    fn find(&mut self, node: usize) -> usize {
        if self.representative[node] == node {
            return node;
        }

        self.representative[node] = self.find(self.representative[node]);
        return self.representative[node];
    }

    fn do_union(&mut self, node_one: usize, node_two: usize) -> bool {
        let node_one = self.find(node_one);
        let node_two = self.find(node_two);

        if node_one == node_two {
            return false;
        }

        if self.size[node_one] > self.size[node_two] {
            self.representative[node_two] = node_one;
            self.size[node_one] += self.size[node_two];
        } else {
            self.representative[node_one] = node_two;
            self.size[node_two] += self.size[node_one];
        }

        return true;
    }
}

impl Solution {
    pub fn find_redundant_connection(edges: Vec<Vec<i32>>) -> Vec<i32> {
        let mut dsu = DSU::new(edges.len());

        for edge in edges.iter() {
            let a = edge[0] - 1;
            let b = edge[1] - 1;
            if !dsu.do_union(a as usize, b as usize) {
                return edge.clone();
            }
        }

        return Vec::new();
    }
}

struct Input {
    edges: Vec<Vec<i32>>,
}

fn main() {
    let inputs = vec![
        Input {
            edges: vec![vec![1, 2], vec![1, 3], vec![2, 3]],
        },
        Input {
            edges: vec![vec![1, 2], vec![2, 3], vec![3, 4], vec![1, 4], vec![1, 5]],
        },
    ];

    for input in inputs {
        let result = Solution::find_redundant_connection(input.edges);
        println!("{result:?}");
    }
}

struct UnionFind {
    roots: Vec<i32>,
    ranks: Vec<i32>,
}

impl UnionFind {
    fn new(n: i32) -> Self {
        let roots: Vec<i32> = (0..n).collect();
        let ranks = vec![1; n as usize];

        return Self { roots, ranks };
    }

    fn find(&mut self, x: i32) -> i32 {
        let i = x as usize;
        if self.roots[i] != x {
            self.roots[i] = self.find(self.roots[i]);
        }

        return self.roots[i];
    }

    fn union(&mut self, x: i32, y: i32) {
        let mut rootx = self.find(x);
        let mut rooty = self.find(y);

        if rootx != rooty {
            let ranks = &mut self.ranks;
            if ranks[rootx as usize] > ranks[rooty as usize] {
                let temp = rootx;
                rootx = rooty;
                rooty = temp;
            }

            let roots = &mut self.roots;
            roots[rootx as usize] = rooty;
            ranks[rooty as usize] += ranks[rootx as usize];
        }
    }
}

struct Solution {}
impl Solution {
    pub fn valid_path(n: i32, edges: Vec<Vec<i32>>, source: i32, destination: i32) -> bool {
        let mut uf = UnionFind::new(n);
        for edges in edges.iter() {
            uf.union(edges[0], edges[1]);
        }

        return uf.find(source) == uf.find(destination);
    }
}

struct Input {
    n: i32,
    edges: Vec<Vec<i32>>,
    source: i32,
    destination: i32,
}

fn main() {
    let inputs = [
        Input {
            n: 3,
            edges: vec![vec![0, 1], vec![1, 2], vec![2, 0]],
            source: 0,
            destination: 2,
        },
        Input {
            n: 6,
            edges: vec![vec![0, 1], vec![0, 2], vec![3, 5], vec![5, 4], vec![4, 3]],
            source: 0,
            destination: 5,
        },
    ];

    for Input {
        n,
        edges,
        source,
        destination,
    } in inputs
    {
        let result = Solution::valid_path(n, edges, source, destination);
        println!("{result}");
    }
}

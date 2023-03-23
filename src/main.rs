mod utils;
use utils::{Solution, UnionFind};

impl Solution {
    pub fn make_connected(n: i32, connections: Vec<Vec<i32>>) -> i32 {
        if (connections.len() as i32) < (n - 1) {
            return -1;
        }

        let mut uf = UnionFind::new(n);
        let mut result = n;

        for conn in connections.iter() {
            let a = conn[0];
            let b = conn[1];
            if uf.find(a) != uf.find(b) {
                result -= 1;
                uf.union(a, b);
            }
        }

        return result - 1;
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
            connections: vec![vec![0, 1], vec![0, 2], vec![1, 2]],
        },
        Input {
            n: 6,
            connections: vec![vec![0, 1], vec![0, 2], vec![0, 3], vec![1, 2], vec![1, 3]],
        },
        Input {
            n: 6,
            connections: vec![vec![0, 1], vec![0, 2], vec![0, 3], vec![1, 2]],
        },
    ];

    for Input { n, connections } in inputs {
        let result = Solution::make_connected(n, connections);
        println!("{result}")
    }
}

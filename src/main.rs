mod utils;
use utils::{Solution, UnionFind};

impl Solution {
    pub fn min_score(n: i32, roads: Vec<Vec<i32>>) -> i32 {
        let mut uf = UnionFind::new(n + 1);
        let mut result = i32::max_value();

        for road in roads.iter() {
            uf.union(road[0], road[1]);
        }

        for road in roads.iter() {
            let a = road[0];
            let d = road[2];
            if uf.find(1) == uf.find(a) {
                result = result.min(d);
            }
        }

        return result;
    }
}

struct Input {
    n: i32,
    roads: Vec<Vec<i32>>,
}

fn main() {
    let inputs = [
        Input {
            n: 4,
            roads: vec![vec![1, 2, 9], vec![2, 3, 6], vec![2, 4, 5], vec![1, 4, 7]],
        },
        Input {
            n: 4,
            roads: vec![vec![1, 2, 2], vec![1, 3, 4], vec![3, 4, 7]],
        },
    ];

    for Input { n, roads } in inputs {
        let result = Solution::min_score(n, roads);
        println!("{result}")
    }
}

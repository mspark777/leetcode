struct Solution {}

impl Solution {
    fn build(edges: Vec<Vec<i32>>, k: i32) -> Vec<i32> {
        let n = edges.len() + 1;
        let mut children = vec![vec![]; n];
        for edge in edges {
            let u = edge[0] as usize;
            let v = edge[1] as usize;
            children[u].push(v as i32);
            children[v].push(u as i32);
        }

        let mut res = vec![0; n];
        for i in 0..n {
            res[i] = Self::dfs(i, -1, &children, k);
        }

        return res;
    }

    fn dfs(node: usize, parent: i32, children: &Vec<Vec<i32>>, k: i32) -> i32 {
        if k < 0 {
            return 0;
        }

        let mut res = 1;
        for child in children[node].iter().cloned() {
            if child == parent {
                continue;
            }
            res += Self::dfs(child as usize, node as i32, children, k - 1);
        }
        return res;
    }

    pub fn max_target_nodes(edges1: Vec<Vec<i32>>, edges2: Vec<Vec<i32>>, k: i32) -> Vec<i32> {
        let n = edges1.len() + 1;
        let count1 = Self::build(edges1, k);
        let count2 = Self::build(edges2, k - 1);
        let max_count2 = *count2.iter().max().unwrap();
        let mut res = vec![0; n];
        for i in 0..n {
            res[i] = count1[i] + max_count2;
        }

        return res;
    }
}

struct Input {
    edges1: Vec<Vec<i32>>,
    edges2: Vec<Vec<i32>>,
    k: i32,
}

fn main() {
    let inputs = vec![
        Input {
            edges1: vec![vec![0, 1], vec![0, 2], vec![2, 3], vec![2, 4]],
            edges2: vec![
                vec![0, 1],
                vec![0, 2],
                vec![0, 3],
                vec![2, 7],
                vec![1, 4],
                vec![4, 5],
                vec![4, 6],
            ],
            k: 2,
        },
        Input {
            edges1: vec![vec![0, 1], vec![0, 2], vec![0, 3], vec![0, 4]],
            edges2: vec![vec![0, 1], vec![1, 2], vec![2, 3]],
            k: 1,
        },
    ];

    for input in inputs {
        let result = Solution::max_target_nodes(input.edges1, input.edges2, input.k);
        println!("{:?}", result);
    }
}

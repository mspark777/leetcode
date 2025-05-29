struct Solution {}

impl Solution {
    fn build(edges: &Vec<Vec<i32>>, color: &mut Vec<usize>) -> Vec<i32> {
        let n = edges.len() + 1;
        let mut children = vec![vec![]; n];
        for edge in edges {
            let u = edge[0] as usize;
            let v = edge[1] as usize;
            children[u].push(v as i32);
            children[v].push(u as i32);
        }

        let res = Self::dfs(0, -1, 0, &children, color);
        return vec![res, (n as i32) - res];
    }

    fn dfs(
        node: usize,
        parent: i32,
        depth: usize,
        children: &Vec<Vec<i32>>,
        color: &mut Vec<usize>,
    ) -> i32 {
        let mut res = 1 - (depth % 2) as i32;
        color[node] = depth % 2;

        for &child in &children[node] {
            if child == parent {
                continue;
            }

            res += Self::dfs(child as usize, node as i32, depth + 1, children, color);
        }

        return res;
    }

    pub fn max_target_nodes(edges1: Vec<Vec<i32>>, edges2: Vec<Vec<i32>>) -> Vec<i32> {
        let n = edges1.len() + 1;
        let m = edges2.len() + 1;
        let mut color1 = vec![0; n];
        let mut color2 = vec![0; m];
        let count1 = Self::build(&edges1, &mut color1);
        let count2 = Self::build(&edges2, &mut color2);
        let mut res = vec![0; n];
        for i in 0..n {
            res[i] = count1[color1[i]] + count2[0].max(count2[1]);
        }

        return res;
    }
}

struct Input {
    edges1: Vec<[i32; 2]>,
    edges2: Vec<[i32; 2]>,
}

fn main() {
    let inputs = vec![
        Input {
            edges1: vec![[0, 1], [0, 2], [2, 3], [2, 4]],
            edges2: vec![[0, 1], [0, 2], [0, 3], [2, 7], [1, 4], [4, 5], [4, 6]],
        },
        Input {
            edges1: vec![[0, 1], [0, 2], [0, 3], [0, 4]],
            edges2: vec![[0, 1], [1, 2], [2, 3]],
        },
    ];

    for input in inputs {
        let result = Solution::max_target_nodes(
            input.edges1.iter().map(|e| e.to_vec()).collect(),
            input.edges2.iter().map(|e| e.to_vec()).collect(),
        );
        println!("{:?}", result);
    }
}

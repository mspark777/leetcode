struct Solution {}
impl Solution {
    pub fn min_time(n: i32, edges: Vec<Vec<i32>>, has_apple: Vec<bool>) -> i32 {
        let mut adj_mat = vec![Vec::<usize>::new(); n as usize];
        for edge in edges.iter() {
            let l = edge[0] as usize;
            let r = edge[1] as usize;

            adj_mat[l].push(r);
            adj_mat[r].push(l);
        }

        return Self::dfs(0, edges.len() + 1, &adj_mat, &has_apple);
    }

    fn dfs(node: usize, prev: usize, adj_mat: &Vec<Vec<usize>>, has_apple: &Vec<bool>) -> i32 {
        let mut total_time = 0;
        for &child in adj_mat[node].iter() {
            if child == prev {
                continue;
            }

            let child_time = Self::dfs(child, node, adj_mat, has_apple);
            if (child_time > 0) || has_apple[child] {
                total_time += child_time + 2;
            }
        }

        return total_time;
    }
}

struct Input {
    n: i32,
    edges: Vec<Vec<i32>>,
    has_apple: Vec<bool>,
}

fn main() {
    let inputs = [
        Input {
            n: 7,
            edges: vec![
                vec![0, 1],
                vec![0, 2],
                vec![1, 4],
                vec![1, 5],
                vec![2, 3],
                vec![2, 6],
            ],
            has_apple: vec![false, false, true, false, true, true, false],
        },
        Input {
            n: 7,
            edges: vec![
                vec![0, 1],
                vec![0, 2],
                vec![1, 4],
                vec![1, 5],
                vec![2, 3],
                vec![2, 6],
            ],
            has_apple: vec![false, false, true, false, false, true, false],
        },
        Input {
            n: 7,
            edges: vec![
                vec![0, 1],
                vec![0, 2],
                vec![1, 4],
                vec![1, 5],
                vec![2, 3],
                vec![2, 6],
            ],
            has_apple: vec![false, false, false, false, false, false, false],
        },
    ];

    for Input {
        n,
        edges,
        has_apple,
    } in inputs
    {
        let result = Solution::min_time(n, edges, has_apple);
        println!("{result}");
    }
}

struct Solution;

impl Solution {
    pub fn find_circle_num(is_connected: Vec<Vec<i32>>) -> i32 {
        let n = is_connected.len();
        if n == 0 {
            return 0;
        }

        let mut seen = vec![false; n];
        let mut result = 0;

        for row in 0..n {
            if !seen[row] {
                result += 1;
                Self::dfs(&is_connected, row, &mut seen);
            }
        }

        result
    }

    fn dfs(graph: &[Vec<i32>], v: usize, seen: &mut [bool]) {
        if seen[v] {
            return;
        }

        seen[v] = true;
        for (i, connected) in graph[v].iter().copied().enumerate() {
            if !seen[i] && (connected == 1) {
                Self::dfs(graph, i, seen);
            }
        }
    }
}

struct Input {
    is_connected: Vec<Vec<i32>>,
}

fn main() {
    let inputs = [Input {
        is_connected: [[1, 1, 0], [1, 1, 0], [0, 0, 1]]
            .map(|v| v.to_vec())
            .to_vec(),
    }];

    for input in inputs.into_iter() {
        let result = Solution::find_circle_num(input.is_connected);
        println!("{:?}", result);
    }
}

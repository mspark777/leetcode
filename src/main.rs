struct Solution {}
impl Solution {
    pub fn all_paths_source_target(graph: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        let mut results = Vec::<Vec<i32>>::new();
        let mut path = Vec::<i32>::new();

        Self::dfs(&graph, &mut results, &mut path, 0);

        return results;
    }

    fn dfs(graph: &Vec<Vec<i32>>, results: &mut Vec<Vec<i32>>, path: &mut Vec<i32>, cur: usize) {
        path.push(cur as i32);

        if cur == (graph.len() - 1) {
            results.push(path.clone());
        } else {
            for &next in graph[cur].iter() {
                Self::dfs(graph, results, path, next as usize);
            }
        }

        path.pop();
    }
}

fn main() {
    let inputs = [
        vec![vec![1, 2], vec![3], vec![3], vec![]],
        vec![vec![4, 3, 1], vec![3, 2, 4], vec![3], vec![4], vec![]],
    ];

    for graph in inputs {
        let result = Solution::all_paths_source_target(graph);
        println!("{result:?}");
    }
}

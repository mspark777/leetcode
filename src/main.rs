mod utils;

use utils::Solution;

impl Solution {
    pub fn is_bipartite(graph: Vec<Vec<i32>>) -> bool {
        const NONE: i32 = 0;
        const RED: i32 = 1;
        //const BLUE: i32 = -1;

        let mut colors = vec![NONE; graph.len()];
        let mut stack = Vec::<usize>::new();

        for i in 0..graph.len() {
            if colors[i] != NONE {
                continue;
            }

            colors[i] = RED;
            stack.push(i);

            while let Some(vertex) = stack.pop() {
                let color = colors[vertex];
                for &avertex in graph[vertex].iter() {
                    let adjacent = avertex as usize;
                    let acolor = colors[adjacent];
                    if acolor == NONE {
                        colors[adjacent] = -color;
                        stack.push(adjacent);
                    } else if color == acolor {
                        return false;
                    }
                }
            }
        }

        return true;
    }
}

fn main() {
    let inputs = [
        vec![vec![1, 2, 3], vec![0, 2], vec![0, 1, 3], vec![0, 2]],
        vec![vec![1, 3], vec![0, 2], vec![1, 3], vec![0, 2]],
    ];

    for graph in inputs {
        let result = Solution::is_bipartite(graph);
        println!("{result}");
    }
}

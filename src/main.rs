mod utils;

use utils::Solution;

impl Solution {
    pub fn find_smallest_set_of_vertices(n: i32, edges: Vec<Vec<i32>>) -> Vec<i32> {
        let mut to_edges = vec![false; n as usize];
        for edge in edges.iter() {
            let to = edge[1] as usize;
            to_edges[to] = true;
        }

        let mut result = Vec::<i32>::new();
        for (i, &isto) in to_edges.iter().enumerate() {
            if !isto {
                result.push(i as i32);
            }
        }

        return result;
    }
}

fn main() {
    let inputs = [
        (
            6,
            vec![vec![0, 1], vec![0, 2], vec![2, 5], vec![3, 4], vec![4, 2]],
        ),
        (
            5,
            vec![vec![0, 1], vec![2, 1], vec![3, 1], vec![1, 4], vec![2, 4]],
        ),
    ];

    for (n, edges) in inputs {
        let result = Solution::find_smallest_set_of_vertices(n, edges);
        println!("{result:?}");
    }
}

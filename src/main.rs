mod utils;

use utils::Solution;

impl Solution {
    pub fn max_probability(
        n: i32,
        edges: Vec<Vec<i32>>,
        succ_prob: Vec<f64>,
        start: i32,
        end: i32,
    ) -> f64 {
        let n = n as usize;
        let mut max_props = vec![0.0; n];
        max_props[start as usize] = 1.0;

        for _ in 0..(n - 1) {
            let mut breakable = true;
            for (j, edge) in edges.iter().enumerate() {
                let u = edge[0] as usize;
                let v = edge[1] as usize;
                let prob = succ_prob[j];
                let umax = max_props[u] * prob;
                if umax > max_props[v] {
                    max_props[v] = umax;
                    breakable = false;
                }

                let vmax = max_props[v] * prob;
                if vmax > max_props[u] {
                    max_props[u] = vmax;
                    breakable = false;
                }
            }

            if breakable {
                break;
            }
        }

        return max_props[end as usize];
    }
}

fn main() {
    let inputs = [
        (
            3,
            vec![vec![0, 1], vec![1, 2], vec![0, 2]],
            vec![0.5, 0.5, 0.2],
            0,
            2,
        ),
        (
            3,
            vec![vec![0, 1], vec![1, 2], vec![0, 2]],
            vec![0.5, 0.5, 0.3],
            0,
            2,
        ),
        (3, vec![vec![0, 1]], vec![0.5], 0, 2),
    ];

    for (n, edges, succ_prob, start, end) in inputs {
        let result = Solution::max_probability(n, edges, succ_prob, start, end);
        println!("{result}");
    }
}

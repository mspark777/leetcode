struct Solution;

use std::collections::HashMap;
use std::collections::HashSet;

struct Edge {
    node: String,
    val: f64,
}

impl Solution {
    fn build_graph(data: Vec<Vec<String>>, vals: Vec<f64>) -> HashMap<String, Vec<Edge>> {
        let mut g = HashMap::new();
        for i in 0..data.len() {
            g.entry(data[i][0].clone())
                .or_insert(Vec::new())
                .push(Edge {
                    node: data[i][1].clone(),
                    val: vals[i],
                });

            g.entry(data[i][1].clone())
                .or_insert(Vec::new())
                .push(Edge {
                    node: data[i][0].clone(),
                    val: 1.0 / vals[i],
                });
        }

        g
    }

    fn search(v1: String, v2: String, g: &HashMap<String, Vec<Edge>>) -> f64 {
        let mut frontier = vec![Edge {
            node: v1.clone(),
            val: 1.0,
        }];
        let mut seen = HashSet::new();
        seen.insert(&v1);

        while !frontier.is_empty() {
            let mut new_frontier = Vec::new();
            for v in frontier {
                if !g.contains_key(&v.node) {
                    continue;
                }

                for n in &g[&v.node] {
                    if n.node == v2 {
                        return v.val * n.val;
                    }

                    if !seen.contains(&n.node) {
                        seen.insert(&n.node);
                        new_frontier.push(Edge {
                            node: n.node.clone(),
                            val: v.val * n.val,
                        });
                    }
                }
            }

            frontier = new_frontier;
        }

        -1.0
    }

    fn answer_query(q: Vec<String>, g: &HashMap<String, Vec<Edge>>) -> f64 {
        match q[1] == q[0] {
            true if g.contains_key(&q[1]) => 1.0,
            true => -1.0,
            _ => Self::search(q[0].clone(), q[1].clone(), g),
        }
    }

    pub fn calc_equation(
        equations: Vec<Vec<String>>,
        values: Vec<f64>,
        queries: Vec<Vec<String>>,
    ) -> Vec<f64> {
        let g = Self::build_graph(equations, values);
        let mut res = Vec::with_capacity(queries.len());
        for q in queries {
            res.push(Self::answer_query(q, &g));
        }

        res
    }
}

struct Input {
    equations: Vec<Vec<String>>,
    values: Vec<f64>,
    queries: Vec<Vec<String>>,
}

fn main() {
    let inputs = [Input {
        equations: [["a", "b"], ["b", "c"]]
            .map(|v| v.map(|v| v.to_string()).to_vec())
            .to_vec(),
        values: [2.0, 3.0].to_vec(),
        queries: [["a", "c"], ["b", "a"], ["a", "e"], ["a", "a"], ["x", "x"]]
            .map(|v| v.map(|v| v.to_string()).to_vec())
            .to_vec(),
    }];

    for input in inputs.into_iter() {
        let result = Solution::calc_equation(input.equations, input.values, input.queries);
        println!("{:?}", result);
    }
}

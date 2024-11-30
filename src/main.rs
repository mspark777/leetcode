use std::collections::HashMap;
use std::collections::VecDeque;

struct Solution {}

impl Solution {
    pub fn valid_arrangement(pairs: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        let mut adjacency_matrix = HashMap::<i32, VecDeque<i32>>::new();
        let mut in_degree = HashMap::<i32, i32>::new();
        let mut out_degree = HashMap::<i32, i32>::new();

        for pair in pairs.iter() {
            let start = pair[0];
            let end = pair[1];
            adjacency_matrix
                .entry(start)
                .or_insert(VecDeque::new())
                .push_back(end);
            *out_degree.entry(start).or_insert(0) += 1;
            *in_degree.entry(end).or_insert(0) += 1;
        }

        let mut nodes = Vec::<i32>::new();
        let mut start_node = -1;
        for (&node, &outd) in out_degree.iter() {
            let def = 0;
            let &ind = in_degree.get(&node).unwrap_or(&def);
            if outd == (ind + 1) {
                start_node = node;
                break;
            }
        }

        if start_node == -1 {
            start_node = pairs[0][0];
        }

        let mut node_stack = Vec::<i32>::new();
        node_stack.push(start_node);

        while let Some(&node) = node_stack.last() {
            let may_adjacency = adjacency_matrix.get_mut(&node);
            if may_adjacency.is_none() {
                nodes.push(node);
                node_stack.pop();
                continue;
            }

            let adjacency = may_adjacency.unwrap();
            if adjacency.is_empty() {
                nodes.push(node);
                node_stack.pop();
            } else {
                let next_node = adjacency.pop_front().unwrap();
                node_stack.push(next_node);
            }
        }

        nodes.reverse();

        let mut result = Vec::<Vec<i32>>::with_capacity(nodes.len() - 1);
        for i in 1..nodes.len() {
            let start = nodes[i - 1];
            let end = nodes[i];
            result.push(vec![start, end]);
        }

        return result;
    }
}

struct Input {
    pairs: Vec<Vec<i32>>,
}

fn main() {
    let inputs = vec![
        Input {
            pairs: vec![vec![5, 1], vec![4, 5], vec![11, 9], vec![9, 4]],
        },
        Input {
            pairs: vec![vec![1, 3], vec![3, 2], vec![2, 1]],
        },
        Input {
            pairs: vec![vec![1, 2], vec![1, 3], vec![2, 1]],
        },
    ];

    for input in inputs {
        let result = Solution::valid_arrangement(input.pairs);
        println!("{result:?}");
    }
}

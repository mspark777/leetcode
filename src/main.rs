struct Solution {}

impl Solution {
    pub fn magnificent_sets(n: i32, edges: Vec<Vec<i32>>) -> i32 {
        let n = n as usize;
        let mut adjacent_list = vec![Vec::<usize>::new(); n];
        let mut parent = vec![n; n];
        let mut depth = vec![0usize; n];

        for edge in edges.iter() {
            let a = (edge[0] - 1) as usize;
            let b = (edge[1] - 1) as usize;

            adjacent_list[a].push(b);
            adjacent_list[b].push(a);
            Self::union(a, b, &mut parent, &mut depth);
        }

        let mut num_of_groups_for_component = std::collections::HashMap::<usize, usize>::new();
        for node in 0..n {
            if let Some(number_of_group) = Self::get_number_of_groups(&adjacent_list, node, n) {
                let root_node = Self::find(node, &mut parent);
                let component = num_of_groups_for_component
                    .entry(root_node)
                    .or_insert(number_of_group);

                *component = number_of_group.max(*component);
            } else {
                return -1;
            }
        }

        let mut result = 0;
        for &group in num_of_groups_for_component.values() {
            result += group as i32;
        }
        return result;
    }

    fn find(mut node: usize, parent: &mut Vec<usize>) -> usize {
        while parent[node] != parent.len() {
            node = parent[node];
        }

        return node;
    }

    fn union(mut node1: usize, mut node2: usize, parent: &mut Vec<usize>, depth: &mut Vec<usize>) {
        node1 = Self::find(node1, parent);
        node2 = Self::find(node2, parent);
        if node1 == node2 {
            return;
        }

        if depth[node1] < depth[node2] {
            let t1 = node1;
            let t2 = node2;
            node1 = t2;
            node2 = t1;
        }

        parent[node2] = node1;
        if depth[node1] == depth[node2] {
            depth[node1] += 1;
        }
    }

    fn get_number_of_groups(
        adjacent_list: &Vec<Vec<usize>>,
        src_node: usize,
        n: usize,
    ) -> Option<usize> {
        let mut queue = vec![src_node];
        let mut layer_seen: Vec<Option<usize>> = vec![None; n];

        layer_seen[src_node] = Some(0);
        let mut deepest_layer = 0;

        while !queue.is_empty() {
            let num_of_nodes_in_layer = queue.len();
            for i in 0..num_of_nodes_in_layer {
                let current_node = queue[i];
                for neighbor in adjacent_list[current_node].iter().cloned() {
                    if let Some(seen) = layer_seen[neighbor] {
                        if seen == deepest_layer {
                            return None;
                        }
                    } else {
                        layer_seen[neighbor] = Some(deepest_layer + 1);
                        queue.push(neighbor);
                    }
                }
            }

            queue = Vec::from_iter(queue.iter().skip(num_of_nodes_in_layer).cloned());
            deepest_layer += 1;
        }

        return Some(deepest_layer);
    }
}

struct Input {
    n: i32,
    edges: Vec<Vec<i32>>,
}

fn main() {
    let inputs = vec![
        Input {
            n: 6,
            edges: vec![
                vec![1, 2],
                vec![1, 4],
                vec![1, 5],
                vec![2, 6],
                vec![2, 3],
                vec![4, 6],
            ],
        },
        Input {
            n: 3,
            edges: vec![vec![1, 2], vec![2, 3], vec![3, 1]],
        },
        Input {
            n: 2,
            edges: vec![vec![1, 2]],
        },
    ];

    for input in inputs {
        let result = Solution::magnificent_sets(input.n, input.edges);
        println!("{result:?}");
    }
}

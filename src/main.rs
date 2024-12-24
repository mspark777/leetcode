struct Solution {}

impl Solution {
    pub fn minimum_diameter_after_merge(edges1: Vec<Vec<i32>>, edges2: Vec<Vec<i32>>) -> i32 {
        let n = edges1.len() + 1;
        let m = edges2.len() + 1;

        let adjacent_list1 = Self::build_adjacent_list(n, &edges1);
        let adjacent_list2 = Self::build_adjacent_list(m, &edges2);

        let diameter1 = Self::find_diameter(n, &adjacent_list1);
        let diameter2 = Self::find_diameter(m, &adjacent_list2);

        let combined_diameter = (((diameter1 as f64) / 2.0).ceil() as i32)
            + (((diameter2 as f64) / 2.0).ceil() as i32)
            + 1;

        return diameter1.max(diameter2).max(combined_diameter);
    }

    fn build_adjacent_list(size: usize, edges: &Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        let mut adjacent_list = vec![Vec::<i32>::new(); size];
        for edge in edges.iter() {
            let a = edge[0];
            let b = edge[1];

            adjacent_list[a as usize].push(b);
            adjacent_list[b as usize].push(a);
        }

        return adjacent_list;
    }

    fn find_diameter(n: usize, adjacent_list: &Vec<Vec<i32>>) -> i32 {
        let mut leaves_queue = std::collections::VecDeque::<i32>::new();
        let mut degrees = vec![0; n];

        for node in 0..n {
            degrees[node] = adjacent_list[node].len();
            if degrees[node] == 1 {
                leaves_queue.push_back(node as i32);
            }
        }

        let mut remaining_nodes = n;
        let mut leaves_layers_removed = 0;

        while remaining_nodes > 2 {
            let size = leaves_queue.len();

            remaining_nodes -= size;
            leaves_layers_removed += 1;

            for _ in 0..size {
                let current_node = leaves_queue.pop_front().unwrap();

                for &neighbor in adjacent_list[current_node as usize].iter() {
                    let n = neighbor as usize;
                    degrees[n] -= 1;
                    if degrees[n] == 1 {
                        leaves_queue.push_back(neighbor);
                    }
                }
            }
        }

        return if remaining_nodes == 2 {
            2 * leaves_layers_removed + 1
        } else {
            2 * leaves_layers_removed
        };
    }
}

struct Input {
    edges1: Vec<Vec<i32>>,
    edges2: Vec<Vec<i32>>,
}

fn main() {
    let inputs = vec![
        Input {
            edges1: vec![vec![0, 1], vec![0, 2], vec![0, 3]],
            edges2: vec![vec![0, 1]],
        },
        Input {
            edges1: vec![
                vec![0, 1],
                vec![0, 2],
                vec![0, 3],
                vec![2, 4],
                vec![2, 5],
                vec![3, 6],
                vec![2, 7],
            ],
            edges2: vec![
                vec![0, 1],
                vec![0, 2],
                vec![0, 3],
                vec![2, 4],
                vec![2, 5],
                vec![3, 6],
                vec![2, 7],
            ],
        },
    ];

    for input in inputs {
        let result = Solution::minimum_diameter_after_merge(input.edges1, input.edges2);
        println!("{result:?}");
    }
}

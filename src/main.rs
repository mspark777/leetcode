struct Solution {}

impl Solution {
    pub fn count_paths(n: i32, roads: Vec<Vec<i32>>) -> i32 {
        let n = n as usize;
        const MOD: i32 = 1000000007;

        let mut graph = vec![Vec::<(usize, usize)>::new(); n];

        for road in roads.iter() {
            let start_node = road[0] as usize;
            let end_node = road[1] as usize;
            let time = road[2] as usize;
            graph[start_node].push((end_node, time));
            graph[end_node].push((start_node, time));
        }

        let mut queue = std::collections::BinaryHeap::<std::cmp::Reverse<(usize, usize)>>::new();
        let mut shortest_time = vec![usize::MAX; n];
        let mut path_count = vec![0; n];

        shortest_time[0] = 0;
        path_count[0] = 1;
        queue.push(std::cmp::Reverse((0, 0)));

        while let Some(std::cmp::Reverse((current_time, current_node))) = queue.pop() {
            if current_time > shortest_time[current_node] {
                continue;
            }

            for &(neighbor_node, road_time) in graph[current_node].iter() {
                let time = current_time + road_time;
                let curr_short = shortest_time[neighbor_node];
                if time < curr_short {
                    shortest_time[neighbor_node] = time;
                    path_count[neighbor_node] = path_count[current_node];
                    queue.push(std::cmp::Reverse((time, neighbor_node)));
                } else if time == curr_short {
                    path_count[neighbor_node] =
                        (path_count[neighbor_node] + path_count[current_node]) % MOD;
                }
            }
        }

        return path_count[n - 1];
    }
}

struct Input {
    n: i32,
    roads: Vec<Vec<i32>>,
}

fn main() {
    let inputs = vec![
        Input {
            n: 7,
            roads: vec![
                vec![0, 6, 7],
                vec![0, 1, 2],
                vec![1, 2, 3],
                vec![1, 3, 3],
                vec![6, 3, 3],
                vec![3, 5, 1],
                vec![6, 5, 1],
                vec![2, 5, 1],
                vec![0, 4, 5],
                vec![4, 6, 2],
            ],
        },
        Input {
            n: 2,
            roads: vec![vec![1, 0, 10]],
        },
    ];

    for input in inputs {
        let result = Solution::count_paths(input.n, input.roads);
        println!("{result:?}");
    }
}

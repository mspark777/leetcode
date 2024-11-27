struct Solution {}

impl Solution {
    pub fn shortest_distance_after_queries(n: i32, queries: Vec<Vec<i32>>) -> Vec<i32> {
        let mut result = vec![0; queries.len()];
        let mut adjacency_list = vec![Vec::<i32>::new(); n as usize];

        for i in 0..(n - 1) {
            adjacency_list[i as usize].push(i + 1);
        }

        for (i, road) in queries.iter().enumerate() {
            let u = road[0];
            let v = road[1];
            adjacency_list[u as usize].push(v);

            result[i] = Self::find_min_distance(&adjacency_list, n);
        }

        return result;
    }

    fn find_min_distance(adjacency_list: &Vec<Vec<i32>>, n: i32) -> i32 {
        let mut dp = vec![0; n as usize];

        for current_node in (0..(n - 1)).rev() {
            let mut min_distance = n;
            for &neighbor in adjacency_list[current_node as usize].iter() {
                min_distance = min_distance.min(dp[neighbor as usize] + 1);
            }
            dp[current_node as usize] = min_distance;
        }

        return dp[0];
    }
}

struct Input {
    n: i32,
    queries: Vec<Vec<i32>>,
}

fn main() {
    let inputs = vec![
        Input {
            n: 5,
            queries: vec![vec![2, 4], vec![0, 2], vec![0, 4]],
        },
        Input {
            n: 4,
            queries: vec![vec![0, 3], vec![0, 2]],
        },
    ];

    for input in inputs {
        let result = Solution::shortest_distance_after_queries(input.n, input.queries);
        println!("{result:?}");
    }
}

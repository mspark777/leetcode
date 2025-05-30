struct Solution {}

impl Solution {
    fn dfs(node: i32, edges: &Vec<i32>, dist: &mut Vec<i32>, visit: &mut Vec<bool>) {
        let node = node as usize;
        visit[node] = true;
        let neighbor = edges[node];
        if (neighbor != -1) && !visit[neighbor as usize] {
            dist[neighbor as usize] = 1 + dist[node];
            Self::dfs(neighbor, edges, dist, visit);
        }
    }

    pub fn closest_meeting_node(edges: Vec<i32>, node1: i32, node2: i32) -> i32 {
        let n = edges.len();
        let mut dist1 = vec![i32::MAX; n];
        let mut dist2 = vec![i32::MAX; n];
        let mut visit1 = vec![false; n];
        let mut visit2 = vec![false; n];

        dist1[node1 as usize] = 0;
        dist2[node2 as usize] = 0;
        Self::dfs(node1, &edges, &mut dist1, &mut visit1);
        Self::dfs(node2, &edges, &mut dist2, &mut visit2);

        let mut min_dist_node = -1;
        let mut min_dist_till_now = i32::MAX;

        for curr_node in 0..n {
            if min_dist_till_now > dist1[curr_node].max(dist2[curr_node]) {
                min_dist_node = curr_node as i32;
                min_dist_till_now = dist1[curr_node].max(dist2[curr_node]);
            }
        }

        return min_dist_node;
    }
}

struct Input {
    edges: Vec<i32>,
    node1: i32,
    node2: i32,
}

fn main() {
    let inputs = vec![
        Input {
            edges: vec![2, 2, 3, -1],
            node1: 0,
            node2: 1,
        },
        Input {
            edges: vec![1, 2, -1],
            node1: 0,
            node2: 2,
        },
    ];

    for input in inputs {
        let result = Solution::closest_meeting_node(input.edges, input.node1, input.node2);
        println!("{:?}", result);
    }
}

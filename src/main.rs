struct Solution {}

impl Solution {
    pub fn find_champion(n: i32, edges: Vec<Vec<i32>>) -> i32 {
        return Self::solve(n as usize, edges);
    }

    fn solve(n: usize, edges: Vec<Vec<i32>>) -> i32 {
        let mut indegree = vec![0; n];
        for edge in edges.iter() {
            let idx = edge[1] as usize;
            indegree[idx] += 1;
        }

        let mut champion = -1;
        let mut champion_count = 0;

        for (team, &count) in indegree.iter().enumerate() {
            if count == 0 {
                champion_count += 1;
                champion = team as i32;
            }
        }

        return if champion_count == 1 { champion } else { -1 };
    }
}

struct Input {
    n: i32,
    edges: Vec<Vec<i32>>,
}

fn main() {
    let inputs = vec![
        Input {
            n: 3,
            edges: vec![vec![0, 1], vec![1, 2]],
        },
        Input {
            n: 4,
            edges: vec![vec![0, 2], vec![1, 3], vec![1, 2]],
        },
    ];

    for input in inputs {
        let result = Solution::find_champion(input.n, input.edges);
        println!("{result}");
    }
}

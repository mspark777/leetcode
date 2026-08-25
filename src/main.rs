struct Solution;

impl Solution {
    pub fn shortest_bridge(grid: Vec<Vec<i32>>) -> i32 {
        let n = grid.len();

        let mut group: Vec<Vec<i32>> = vec![vec![-1; n]; n];
        let mut group_ind = 0;

        for i in 0..n {
            for j in 0..n {
                if group[i][j] == -1 && grid[i][j] == 1 {
                    Solution::dfs(&grid, (i, j), &mut group, group_ind);
                    group_ind += 1;
                }
            }
        }

        let groups = [0, 1]
            .iter()
            .map(|group_ind| {
                group
                    .iter()
                    .enumerate()
                    .flat_map(|(i, v)| {
                        v.iter()
                            .enumerate()
                            .filter(|(_, g)| g == &group_ind)
                            .map(|(j, _)| (i as i32, j as i32))
                            .collect::<Vec<_>>()
                    })
                    .collect::<Vec<_>>()
            })
            .collect::<Vec<_>>();

        let first_group = &groups[0];
        let second_group = &groups[1];

        let mut result = i32::MAX;

        for (fi, fj) in first_group {
            for (si, sj) in second_group {
                result = result.min((fi - si).abs() + (fj - sj).abs() - 1);
            }
        }

        result
    }

    fn dfs(grid: &Vec<Vec<i32>>, cur: (usize, usize), group: &mut Vec<Vec<i32>>, group_ind: i32) {
        let n = grid.len() as i32;
        group[cur.0][cur.1] = group_ind;

        for (di, dj) in [(1, 0), (-1, 0), (0, 1), (0, -1)] {
            let to_i = cur.0 as i32 + di;
            let to_j = cur.1 as i32 + dj;

            if to_i >= 0 && to_i < n && to_j >= 0 && to_j < n {
                let to_i = to_i as usize;
                let to_j = to_j as usize;

                if group[to_i][to_j] == -1 && grid[to_i][to_j] == 1 {
                    Solution::dfs(grid, (to_i, to_j), group, group_ind);
                }
            }
        }
    }
}

struct Input {
    grid: Vec<Vec<i32>>,
}

fn main() {
    let inputs = [Input {
        grid: [
            [1, 1, 1, 1, 1],
            [1, 0, 0, 0, 1],
            [1, 0, 1, 0, 1],
            [1, 0, 0, 0, 1],
            [1, 1, 1, 1, 1],
        ]
        .map(|v| v.to_vec())
        .to_vec(),
    }];

    for input in inputs {
        let result = Solution::shortest_bridge(input.grid);
        println!("{:?}", result);
    }
}

struct Solution {}

impl Solution {
    pub fn query_results(_limit: i32, queries: Vec<Vec<i32>>) -> Vec<i32> {
        let n = queries.len();
        let mut result = vec![0; n];
        let mut color_map = std::collections::HashMap::<i32, i32>::with_capacity(n);
        let mut ball_map = std::collections::HashMap::<i32, i32>::with_capacity(n);

        for (i, query) in queries.iter().enumerate() {
            let ball = query[0];
            if let Some(prev_color) = ball_map.get(&ball) {
                let prev_count = color_map.get_mut(prev_color).unwrap();
                if (*prev_count) > 1 {
                    *prev_count -= 1;
                } else {
                    color_map.remove(prev_color);
                }
            }

            let color = query[1];
            ball_map.insert(ball, color);
            let count = color_map.entry(color).or_insert(0);
            *count += 1;
            result[i] = color_map.len() as i32;
        }

        return result;
    }
}

struct Input {
    limit: i32,
    queries: Vec<Vec<i32>>,
}

fn main() {
    let inputs = vec![
        Input {
            limit: 4,
            queries: vec![vec![1, 4], vec![2, 5], vec![1, 3], vec![3, 4]],
        },
        Input {
            limit: 4,
            queries: vec![vec![0, 1], vec![1, 2], vec![2, 2], vec![3, 4], vec![4, 5]],
        },
    ];

    for input in inputs {
        let result = Solution::query_results(input.limit, input.queries);
        println!("{result:?}");
    }
}

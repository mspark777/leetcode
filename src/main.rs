struct Solution {}

impl Solution {
    pub fn leftmost_building_queries(heights: Vec<i32>, queries: Vec<Vec<i32>>) -> Vec<i32> {
        let mut store_queries = vec![Vec::<(i32, i32)>::new(); heights.len()];
        let mut result = vec![-1; queries.len()];

        for (i, query) in queries.iter().enumerate() {
            let a = query[0] as usize;
            let b = query[1] as usize;
            if (a < b) && (heights[a] < heights[b]) {
                result[i] = query[1];
            } else if (a > b) && (heights[a] > heights[b]) {
                result[i] = query[0];
            } else if a == b {
                result[i] = query[0];
            } else {
                store_queries[a.max(b)].push((heights[a].max(heights[b]), i as i32));
            }
        }

        let mut queue =
            std::collections::BinaryHeap::<std::cmp::Reverse<(i32, i32)>>::with_capacity(
                queries.len(),
            );
        for (i, &height) in heights.iter().enumerate() {
            while let Some(std::cmp::Reverse(top)) = queue.peek() {
                if top.0 < height {
                    result[top.1 as usize] = i as i32;
                    queue.pop();
                } else {
                    break;
                }
            }

            for element in store_queries[i].iter() {
                queue.push(std::cmp::Reverse(*element));
            }
        }

        return result;
    }
}

struct Input {
    heights: Vec<i32>,
    queries: Vec<Vec<i32>>,
}

fn main() {
    let inputs = vec![
        Input {
            heights: vec![6, 4, 8, 5, 2, 7],
            queries: vec![vec![0, 1], vec![0, 3], vec![2, 4], vec![3, 4], vec![2, 2]],
        },
        Input {
            heights: vec![5, 3, 8, 2, 6, 1, 4, 6],
            queries: vec![vec![0, 7], vec![3, 5], vec![5, 2], vec![3, 0], vec![1, 6]],
        },
    ];

    for input in inputs {
        let result = Solution::leftmost_building_queries(input.heights, input.queries);
        println!("{result:?}");
    }
}

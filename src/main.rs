struct Solution {}

impl Solution {
    pub fn check_if_prerequisite(
        num_courses: i32,
        prerequisites: Vec<Vec<i32>>,
        queries: Vec<Vec<i32>>,
    ) -> Vec<bool> {
        let num_courses = num_courses as usize;
        let mut is_prerequisite = vec![vec![false; num_courses]; num_courses];
        for edge in prerequisites.iter() {
            is_prerequisite[edge[0] as usize][edge[1] as usize] = true;
        }

        for intermediate in 0..num_courses {
            for src in 0..num_courses {
                for target in 0..num_courses {
                    is_prerequisite[src][target] = is_prerequisite[src][target]
                        || (is_prerequisite[src][intermediate]
                            && is_prerequisite[intermediate][target]);
                }
            }
        }

        let mut result = vec![false; queries.len()];
        for (i, query) in queries.iter().enumerate() {
            result[i] = is_prerequisite[query[0] as usize][query[1] as usize];
        }
        return result;
    }
}

struct Input {
    num_courses: i32,
    prerequisites: Vec<Vec<i32>>,
    queries: Vec<Vec<i32>>,
}

fn main() {
    let inputs = vec![
        Input {
            num_courses: 2,
            prerequisites: vec![vec![1, 0]],
            queries: vec![vec![0, 1], vec![1, 0]],
        },
        Input {
            num_courses: 2,
            prerequisites: vec![],
            queries: vec![vec![1, 0], vec![0, 1]],
        },
        Input {
            num_courses: 3,
            prerequisites: vec![vec![1, 2], vec![1, 0], vec![2, 0]],
            queries: vec![vec![1, 0], vec![1, 2]],
        },
    ];

    for input in inputs {
        let result =
            Solution::check_if_prerequisite(input.num_courses, input.prerequisites, input.queries);
        println!("{result:?}");
    }
}

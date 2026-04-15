struct Solution;

impl Solution {
    pub fn find_order(num_courses: i32, prerequisites: Vec<Vec<i32>>) -> Vec<i32> {
        use std::collections::VecDeque;

        let num_courses = num_courses as usize;
        let mut adjacency_nodes = vec![Vec::<usize>::new(); num_courses];
        let mut in_degree = vec![0; num_courses];

        for require in prerequisites.iter() {
            let target = require[0] as usize;
            let course = require[1] as usize;

            adjacency_nodes[course].push(target);
            in_degree[target] += 1;
        }

        let mut queue = VecDeque::<usize>::new();
        for (i, degree) in in_degree.iter().copied().enumerate() {
            if degree == 0 {
                queue.push_back(i);
            }
        }

        let mut courses = Vec::<i32>::new();
        while let Some(node) = queue.pop_back() {
            courses.push(node as i32);
            for target in adjacency_nodes[node].iter().copied() {
                in_degree[target] -= 1;
                if in_degree[target] == 0 {
                    queue.push_back(target);
                }
            }
        }

        match courses.len() == num_courses {
            true => courses,
            _ => Vec::new(),
        }
    }
}

struct Input {
    num_courses: i32,
    prerequisites: Vec<Vec<i32>>,
}

fn main() {
    let inputs = [
        Input {
            num_courses: 2,
            prerequisites: [[1, 0]].map(|v| v.to_vec()).to_vec(),
        },
        Input {
            num_courses: 4,
            prerequisites: [[1, 0], [2, 0], [3, 1], [3, 2]]
                .map(|v| v.to_vec())
                .to_vec(),
        },
    ];

    for input in inputs {
        let result = Solution::find_order(input.num_courses, input.prerequisites);
        println!("{:?}", result);
    }
}

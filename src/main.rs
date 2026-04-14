struct Solution;

impl Solution {
    pub fn can_finish(num_courses: i32, prerequisites: Vec<Vec<i32>>) -> bool {
        use std::collections::VecDeque;

        let num_courses = num_courses as usize;
        let mut in_degree = vec![0; num_courses];
        let mut adjacents = vec![Vec::<usize>::new(); num_courses];

        for prereq in prerequisites {
            adjacents[prereq[1] as usize].push(prereq[0] as usize);
            in_degree[prereq[0] as usize] += 1;
        }

        let mut queue: VecDeque<usize> = VecDeque::new();
        for (i, degree) in in_degree.iter().copied().enumerate() {
            if degree == 0 {
                queue.push_back(i);
            }
        }

        let mut processed_courses = 0;
        while let Some(course) = queue.pop_front() {
            processed_courses += 1;
            for next_course in adjacents[course].iter().copied() {
                in_degree[next_course] -= 1;
                if in_degree[next_course] == 0 {
                    queue.push_back(next_course);
                }
            }
        }

        processed_courses == num_courses
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
            num_courses: 2,
            prerequisites: [[1, 0], [0, 1]].map(|v| v.to_vec()).to_vec(),
        },
    ];

    for input in inputs {
        let result = Solution::can_finish(input.num_courses, input.prerequisites);
        println!("{:?}", result);
    }
}

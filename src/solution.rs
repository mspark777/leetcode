use std::collections::BinaryHeap;

pub struct Solution {}

impl Solution {
    pub fn schedule_course(courses: Vec<Vec<i32>>) -> i32 {
        let mut courses = courses;
        let mut queue = BinaryHeap::<i32>::with_capacity(courses.len());

        courses.sort_unstable_by(|a, b| a[1].cmp(&b[1]));

        let mut time = 0;
        for course in courses.iter() {
            let duration = course[0];
            let last = course[1];
            let new_time = time + duration;
            if new_time <= last {
                time = new_time;
                queue.push(duration);
            } else if let Some(mut top) = queue.peek_mut() {
                let long = *top;
                if long > duration {
                    time += duration - long;
                    *top = duration;
                }
            }
        }

        queue.len() as i32
    }
}

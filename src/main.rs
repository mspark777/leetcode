struct QueueNode {
    current_gain: f64,
    passes: i32,
    total_students: i32,
}

impl Ord for QueueNode {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        if self.current_gain < other.current_gain {
            return std::cmp::Ordering::Less;
        } else if self.current_gain > other.current_gain {
            return std::cmp::Ordering::Greater;
        }

        return (self.passes, self.total_students).cmp(&(other.passes, other.total_students));
    }
}
impl PartialOrd for QueueNode {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for QueueNode {
    fn eq(&self, other: &Self) -> bool {
        return match self.cmp(other) {
            std::cmp::Ordering::Equal => true,
            _ => false,
        };
    }
}

impl Eq for QueueNode {}

struct Solution {}

impl Solution {
    pub fn max_average_ratio(classes: Vec<Vec<i32>>, extra_students: i32) -> f64 {
        let mut queue = std::collections::BinaryHeap::<QueueNode>::with_capacity(classes.len());
        for cls in classes.iter() {
            let passes = cls[0];
            let total_students = cls[1];
            let current_gain = Self::calculate_gain(passes, total_students);
            queue.push(QueueNode {
                current_gain,
                passes,
                total_students,
            });
        }

        let mut extra = extra_students;
        while extra > 0 {
            if queue.is_empty() {
                break;
            }

            let top = queue.pop().unwrap();
            let passes = top.passes + 1;
            let total_students = top.total_students + 1;
            let current_gain = Self::calculate_gain(passes, total_students);
            queue.push(QueueNode {
                current_gain,
                passes,
                total_students,
            });
            extra -= 1;
        }

        let mut total_pass_ratio = 0f64;

        while let Some(top) = queue.pop() {
            total_pass_ratio += Self::as_f64(top.passes) / Self::as_f64(top.total_students);
        }

        return total_pass_ratio / (classes.len() as f64);
    }

    fn calculate_gain(passes: i32, total_students: i32) -> f64 {
        return Self::as_f64(passes + 1) / Self::as_f64(total_students + 1)
            - Self::as_f64(passes) / Self::as_f64(total_students);
    }

    fn as_f64(i: i32) -> f64 {
        return i as f64;
    }
}

struct Input {
    classes: Vec<Vec<i32>>,
    extra_students: i32,
}

fn main() {
    let inputs = vec![
        Input {
            classes: vec![vec![1, 2], vec![3, 5], vec![2, 2]],
            extra_students: 2,
        },
        Input {
            classes: vec![vec![2, 4], vec![3, 9], vec![4, 5], vec![2, 10]],
            extra_students: 4,
        },
    ];

    for input in inputs {
        let result = Solution::max_average_ratio(input.classes, input.extra_students);
        println!("{result}");
    }
}

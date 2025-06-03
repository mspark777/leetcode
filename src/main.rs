struct Solution {}

impl Solution {
    pub fn max_candies(
        status: Vec<i32>,
        candies: Vec<i32>,
        keys: Vec<Vec<i32>>,
        contained_boxes: Vec<Vec<i32>>,
        initial_boxes: Vec<i32>,
    ) -> i32 {
        let n = status.len();
        let mut can_open = status.iter().map(|&s| s == 1).collect::<Vec<_>>();
        let mut has_box = vec![false; n];
        let mut used = vec![false; n];
        let mut queue = std::collections::VecDeque::<usize>::new();
        let mut result = 0;
        for box_id in initial_boxes {
            let box_id = box_id as usize;
            has_box[box_id] = true;
            if can_open[box_id] {
                queue.push_back(box_id);
                used[box_id] = true;
                result += candies[box_id];
            }
        }

        while let Some(big_box) = queue.pop_front() {
            for key in keys[big_box].iter().cloned() {
                let key = key as usize;
                can_open[key] = true;
                if !used[key] && has_box[key] {
                    queue.push_back(key);
                    used[key] = true;
                    result += candies[key];
                }
            }
            for box_id in contained_boxes[big_box].iter().cloned() {
                let box_id = box_id as usize;
                has_box[box_id] = true;
                if !used[box_id] && can_open[box_id] {
                    queue.push_back(box_id);
                    used[box_id as usize] = true;
                    result += candies[box_id as usize];
                }
            }
        }

        return result;
    }
}

struct Input {
    status: Vec<i32>,
    candies: Vec<i32>,
    keys: Vec<Vec<i32>>,
    contained_boxes: Vec<Vec<i32>>,
    initial_boxes: Vec<i32>,
}

fn main() {
    let inputs = vec![
        Input {
            status: vec![1, 0, 1, 0],
            candies: vec![7, 5, 4, 100],
            keys: vec![vec![], vec![], vec![1], vec![]],
            contained_boxes: vec![vec![1, 2], vec![3], vec![], vec![]],
            initial_boxes: vec![0],
        },
        Input {
            status: vec![1, 0, 0, 0, 0, 0],
            candies: vec![1, 1, 1, 1, 1, 1],
            keys: vec![vec![1, 2, 3, 4, 5], vec![], vec![], vec![], vec![], vec![]],
            contained_boxes: vec![vec![1, 2, 3, 4, 5], vec![], vec![], vec![], vec![], vec![]],
            initial_boxes: vec![0],
        },
    ];

    for input in inputs {
        let result = Solution::max_candies(
            input.status,
            input.candies,
            input.keys,
            input.contained_boxes,
            input.initial_boxes,
        );
        println!("{:?}", result);
    }
}

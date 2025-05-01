struct Solution {}

impl Solution {
    pub fn max_task_assign(tasks: Vec<i32>, workers: Vec<i32>, pills: i32, strength: i32) -> i32 {
        let n = tasks.len();
        let m = workers.len();
        let mut tasks = tasks;
        let mut workers = workers;

        tasks.sort();
        workers.sort();

        let mut left = 1;
        let mut right = m.min(n);
        let mut result = 0usize;
        while left <= right {
            let mid = (left + right) / 2;
            if Self::check(mid, pills, m, &workers, strength, &tasks) {
                result = mid;
                left = mid + 1;
            } else {
                right = mid - 1;
            }
        }

        return result as i32;
    }

    fn check(
        mid: usize,
        p: i32,
        m: usize,
        workers: &Vec<i32>,
        strength: i32,
        tasks: &Vec<i32>,
    ) -> bool {
        let mut p = p;
        let mut ws = std::collections::VecDeque::<i32>::new();
        let mut ptr = m - 1;
        for i in (0..mid).rev() {
            while ptr as i32 >= (mid - mid) as i32 && workers[ptr] + strength >= tasks[i] {
                ws.push_front(workers[ptr]);
                ptr -= 1;
            }

            if ws.is_empty() {
                return false;
            }

            if *ws.back().unwrap() >= tasks[i] {
                ws.pop_back();
            } else {
                if p == 0 {
                    return false;
                }

                p -= 1;
                ws.pop_front();
            }
        }

        return true;
    }
}

struct Input {
    tasks: Vec<i32>,
    workers: Vec<i32>,
    pills: i32,
    strength: i32,
}

fn main() {
    let inputs = vec![
        Input {
            tasks: [3, 2, 1].to_vec(),
            workers: [3, 2, 1].to_vec(),
            pills: 1,
            strength: 1,
        },
        Input {
            tasks: [5, 4].to_vec(),
            workers: [0, 0, 0].to_vec(),
            pills: 1,
            strength: 5,
        },
        Input {
            tasks: [10, 15, 30].to_vec(),
            workers: [0, 10, 10, 10, 10].to_vec(),
            pills: 3,
            strength: 10,
        },
    ];

    for input in inputs {
        let result =
            Solution::max_task_assign(input.tasks, input.workers, input.pills, input.strength);
        println!("{result:?}");
    }
}

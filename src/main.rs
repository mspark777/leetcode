struct RecentCounter {
    queue: std::collections::VecDeque<i32>,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl RecentCounter {
    fn new() -> Self {
        return Self {
            queue: std::collections::VecDeque::new(),
        };
    }

    fn ping(&mut self, t: i32) -> i32 {
        self.queue.push_back(t);
        let threshold = t - 3000;
        while let Some(&num) = self.queue.front() {
            if num < threshold {
                self.queue.pop_front();
            } else {
                break;
            }
        }

        return self.queue.len() as i32;
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

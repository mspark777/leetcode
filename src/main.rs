struct NumberContainers {
    number_to_indices:
        std::collections::HashMap<i32, std::collections::BinaryHeap<std::cmp::Reverse<usize>>>,
    index_to_numbers: std::collections::HashMap<usize, i32>,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl NumberContainers {
    fn new() -> Self {
        return Self {
            number_to_indices: std::collections::HashMap::new(),
            index_to_numbers: std::collections::HashMap::new(),
        };
    }

    fn change(&mut self, index: i32, number: i32) {
        let index = index as usize;
        *self.index_to_numbers.entry(index).or_insert(number) = number;
        let queue = self
            .number_to_indices
            .entry(number)
            .or_insert(std::collections::BinaryHeap::new());
        queue.push(std::cmp::Reverse(index));
    }

    fn find(&mut self, number: i32) -> i32 {
        if let None = self.number_to_indices.get(&number) {
            return -1;
        }

        if let Some(queue) = self.number_to_indices.get_mut(&number) {
            while let Some(std::cmp::Reverse(top)) = queue.peek() {
                if let Some(&n) = self.index_to_numbers.get(top) {
                    if n == number {
                        return (*top) as i32;
                    }

                    queue.pop();
                } else {
                    queue.pop();
                }
            }
        }

        return -1;
    }
}

/**
 * Your NumberContainers object will be instantiated and called as such:
 * let obj = NumberContainers::new();
 * obj.change(index, number);
 * let ret_2: i32 = obj.find(number);
 */

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

struct Solution;

impl Solution {
    pub fn minimum_index(capacity: Vec<i32>, item_size: i32) -> i32 {
        let mut result = -1;
        let mut min_capacity = 101;
        for (i, c) in capacity.into_iter().enumerate() {
            if item_size > c {
                continue;
            }

            if c < min_capacity {
                min_capacity = c;
                result = i as i32;
            }
        }

        result
    }
}

struct Input {
    capacity: Vec<i32>,
    item_size: i32,
}

fn main() {
    let inputs = [Input {
        capacity: [1, 5, 3, 7].to_vec(),
        item_size: 3,
    }];

    for input in inputs {
        let result = Solution::minimum_index(input.capacity, input.item_size);
        println!("{:?}", result);
    }
}

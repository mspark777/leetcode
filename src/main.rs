struct Solution;

impl Solution {
    pub fn recover_order(order: Vec<i32>, friends: Vec<i32>) -> Vec<i32> {
        use std::collections::HashSet;

        let friend_set = HashSet::<i32>::from_iter(friends);
        let mut result = Vec::<i32>::with_capacity(friend_set.len());
        for id in order {
            if friend_set.contains(&id) {
                result.push(id);
            }
        }

        result
    }
}

struct Input {
    order: Vec<i32>,
    friends: Vec<i32>,
}

fn main() {
    let inputs = [
        Input {
            order: [3, 1, 2, 5, 4].to_vec(),
            friends: [1, 3, 4].to_vec(),
        },
        Input {
            order: [1, 4, 5, 3, 2].to_vec(),
            friends: [2, 5].to_vec(),
        },
    ];

    for input in inputs {
        let result = Solution::recover_order(input.order, input.friends);
        println!("{:?}", result);
    }
}

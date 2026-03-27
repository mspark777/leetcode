struct Solution;

impl Solution {
    pub fn missing_multiple(nums: Vec<i32>, k: i32) -> i32 {
        use std::collections::HashSet;
        let num_set = HashSet::<i32>::from_iter(nums);

        for i in (k..201).step_by(k as usize) {
            if !num_set.contains(&i) {
                return i;
            }
        }

        0
    }
}

struct Input {
    nums: Vec<i32>,
    k: i32,
}

fn main() {
    let inputs = [
        Input {
            nums: [8, 2, 3, 4, 6].to_vec(),
            k: 2,
        },
        Input {
            nums: [1, 4, 7, 10, 15].to_vec(),
            k: 5,
        },
    ];

    for input in inputs {
        let result = Solution::missing_multiple(input.nums, input.k);
        println!("{:?}", result);
    }
}

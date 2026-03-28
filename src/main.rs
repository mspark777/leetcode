struct Solution;

impl Solution {
    pub fn minimum_distance(nums: Vec<i32>) -> i32 {
        use std::collections::HashMap;

        let mut min_distance = i32::MAX;
        let mut index_map = HashMap::<i32, Vec<i32>>::new();

        for (i, num) in nums.into_iter().enumerate() {
            let i = i as i32;

            let index_list = index_map
                .entry(num)
                .and_modify(|l| l.push(i))
                .or_insert(vec![i]);

            let list_len = index_list.len();
            if list_len < 3 {
                continue;
            }

            let i = index_list[list_len - 3];
            let k = index_list[list_len - 1];
            min_distance = min_distance.min(k - i);
        }

        match min_distance {
            i32::MAX => -1,
            _ => min_distance * 2,
        }
    }
}

struct Input {
    nums: Vec<i32>,
}

fn main() {
    let inputs = [
        Input {
            nums: [1, 2, 1, 1, 3].to_vec(),
        },
        Input {
            nums: [1, 1, 2, 3, 2, 1, 2].to_vec(),
        },
        Input { nums: [1].to_vec() },
    ];

    for input in inputs {
        let result = Solution::minimum_distance(input.nums);
        println!("{:?}", result);
    }
}

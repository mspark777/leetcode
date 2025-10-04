struct Solution {}

impl Solution {
    pub fn most_frequent(nums: Vec<i32>, key: i32) -> i32 {
        let mut map = std::collections::HashMap::<i32, i32>::new();
        let mut max = 0;
        let mut result = 0;
        for i in 1..nums.len() {
            let prev = nums[i - 1];
            let num = nums[i];
            if prev != key {
                continue;
            }

            let count = *map.entry(num).and_modify(|count| *count += 1).or_insert(1);
            if count > max {
                max = count;
                result = num;
            }
        }
        result
    }
}

struct Input {
    nums: Vec<i32>,
    key: i32,
}

fn main() {
    let inputs = [
        Input {
            nums: [1, 100, 200, 1, 100].to_vec(),
            key: 1,
        },
        Input {
            nums: [2, 2, 2, 2, 3].to_vec(),
            key: 2,
        },
    ];

    for input in inputs {
        let result = Solution::most_frequent(input.nums, input.key);
        println!("{:?}", result);
    }
}

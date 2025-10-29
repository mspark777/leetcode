struct Solution {}

impl Solution {
    pub fn most_frequent_even(nums: Vec<i32>) -> i32 {
        let mut counts = std::collections::HashMap::<i32, i32>::with_capacity(nums.len());
        let mut max_count = -1;
        let mut result = -1;

        for num in nums.iter().cloned() {
            if num & 1 == 1 {
                continue;
            }

            let count = *counts.entry(num).and_modify(|c| *c += 1).or_insert(1);
            if result < 0 {
                result = num;
                max_count = count;
                continue;
            }

            if count == max_count {
                if num < result {
                    result = num;
                }
            } else if count > max_count {
                max_count = count;
                result = num;
            }
        }

        result
    }
}

struct Input {
    nums: Vec<i32>,
}

fn main() {
    let inputs = [
        Input {
            nums: [0, 1, 2, 2, 4, 4, 1].to_vec(),
        },
        Input {
            nums: [4, 4, 4, 9, 2, 4].to_vec(),
        },
        Input {
            nums: [29, 47, 21, 41, 13, 37, 25, 7].to_vec(),
        },
    ];

    for input in inputs {
        let result = Solution::most_frequent_even(input.nums);
        println!("{:?}", result);
    }
}

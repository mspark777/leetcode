struct Solution;

impl Solution {
    pub fn max_k_distinct(nums: Vec<i32>, k: i32) -> Vec<i32> {
        use std::collections::HashSet;

        let k = k as usize;
        let mut nums = nums;

        nums.sort_by_key(|n| -n);
        let mut checked = HashSet::<i32>::from_iter(nums.iter().copied());
        let mut result = Vec::<i32>::with_capacity(checked.len());
        for num in nums {
            if checked.contains(&num) {
                result.push(num);
                checked.remove(&num);

                if result.len() >= k {
                    break;
                }
            }
        }

        result
    }
}

struct Input {
    nums: Vec<i32>,
    k: i32,
}

fn main() {
    let inputs = [
        Input {
            nums: [84, 93, 100, 77, 90].to_vec(),
            k: 3,
        },
        Input {
            nums: [84, 93, 100, 77, 93].to_vec(),
            k: 3,
        },
        Input {
            nums: [1, 1, 1, 2, 2, 2].to_vec(),
            k: 6,
        },
    ];

    for input in inputs {
        let result = Solution::max_k_distinct(input.nums, input.k);
        println!("{:?}", result);
    }
}

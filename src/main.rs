struct Solution;

impl Solution {
    pub fn min_operations(nums: Vec<i32>, k: i32) -> i32 {
        let mut collections = std::collections::HashSet::<i32>::with_capacity(k as usize);
        let mut result = 0;

        for num in nums.into_iter().rev() {
            result += 1;
            if num <= k {
                collections.insert(num);
            }

            if collections.len() == k as usize {
                break;
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
            nums: [3, 1, 5, 4, 2].to_vec(),
            k: 2,
        },
        Input {
            nums: [3, 1, 5, 4, 2].to_vec(),
            k: 5,
        },
        Input {
            nums: [3, 2, 5, 3, 1].to_vec(),
            k: 3,
        },
    ];

    for input in inputs {
        let result = Solution::min_operations(input.nums, input.k);
        println!("{:?}", result);
    }
}

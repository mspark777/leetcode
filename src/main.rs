struct Solution {}

impl Solution {
    pub fn min_operations(nums: Vec<i32>, k: i32) -> i32 {
        let mut num_set = std::collections::HashSet::<i32>::with_capacity(nums.len());
        for num in nums.iter().cloned() {
            if num < k {
                return -1;
            } else if num > k {
                num_set.insert(num);
            }
        }

        return num_set.len() as i32;
    }
}

struct Input {
    nums: Vec<i32>,
    k: i32,
}

fn main() {
    let inputs = vec![
        Input {
            nums: vec![5, 2, 5, 4, 5],
            k: 2,
        },
        Input {
            nums: vec![2, 1, 2],
            k: 2,
        },
        Input {
            nums: vec![9, 7, 5, 3],
            k: 1,
        },
    ];

    for input in inputs {
        let result = Solution::min_operations(input.nums, input.k);
        println!("{result:?}");
    }
}

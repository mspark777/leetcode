/*
leetcode
 */

struct Solution {}
impl Solution {
    pub fn shuffle(nums: Vec<i32>, n: i32) -> Vec<i32> {
        let mut result = vec![0; nums.len()];

        let n = n as usize;
        for i in 0..n {
            let j = i * 2;
            result[j] = nums[i];
            result[j + 1] = nums[n + i];
        }

        return result;
    }
}

struct Input {
    nums: Vec<i32>,
    n: i32,
}

fn main() {
    let inputs: Vec<Input> = vec![
        Input {
            nums: vec![2, 5, 1, 3, 4, 7],
            n: 3,
        },
        Input {
            nums: vec![1, 2, 3, 4, 4, 3, 2, 1],
            n: 4,
        },
    ];

    for Input { nums, n } in inputs {
        let result = Solution::shuffle(nums, n);
        println!("{result:?}");
    }
}

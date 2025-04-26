struct Solution {}

impl Solution {
    pub fn count_subarrays(nums: Vec<i32>, min_k: i32, max_k: i32) -> i64 {
        let mut result = 0i64;
        let mut bad_idx = -1i64;
        let mut left_idx = -1i64;
        let mut right_idx = -1i64;

        for (i, num) in nums.iter().cloned().enumerate() {
            if (num < min_k) || (num > max_k) {
                bad_idx = i as i64;
            }

            if num == min_k {
                left_idx = i as i64;
            }

            if num == max_k {
                right_idx = i as i64;
            }

            result += 0.max(left_idx.min(right_idx) - bad_idx);
        }

        return result;
    }
}

struct Input {
    nums: Vec<i32>,
    min_k: i32,
    max_k: i32,
}

fn main() {
    let inputs = vec![
        Input {
            nums: vec![1, 3, 5, 2, 7, 5],
            min_k: 1,
            max_k: 5,
        },
        Input {
            nums: vec![1, 1, 1, 1],
            min_k: 1,
            max_k: 1,
        },
    ];

    for input in inputs {
        let result = Solution::count_subarrays(input.nums, input.min_k, input.max_k);
        println!("{result:?}");
    }
}

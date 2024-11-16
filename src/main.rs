struct Solution {}

impl Solution {
    pub fn results_array(nums: Vec<i32>, k: i32) -> Vec<i32> {
        return Self::solve(nums, k as usize);
    }

    fn solve(nums: Vec<i32>, k: usize) -> Vec<i32> {
        if k == 1 {
            return nums;
        }

        let nums_len = nums.len();
        let mut result = vec![-1; nums_len - k + 1];
        let mut consecutive_count = 1;

        for i in 0..(nums_len - 1) {
            let current = nums[i] + 1;
            let next = nums[i + 1];
            if current == next {
                consecutive_count += 1;
            } else {
                consecutive_count = 1;
            }

            if consecutive_count >= k {
                result[i + 2 - k] = nums[i + 1];
            }
        }

        return result;
    }
}

struct Input {
    nums: Vec<i32>,
    k: i32,
}

fn main() {
    let inputs = vec![
        Input {
            nums: vec![1, 2, 3, 4, 3, 2, 5],
            k: 3,
        },
        Input {
            nums: vec![2, 2, 2, 2, 2],
            k: 4,
        },
        Input {
            nums: vec![3, 2, 3, 2, 3, 2],
            k: 2,
        },
    ];

    for input in inputs {
        let result = Solution::results_array(input.nums, input.k);
        println!("{result:?}");
    }
}

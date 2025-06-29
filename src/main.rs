struct Solution {}

impl Solution {
    pub fn num_subseq(nums: Vec<i32>, target: i32) -> i32 {
        const MOD: i32 = 1_000_000_007;
        let n = nums.len();
        let mut nums = nums;
        nums.sort_unstable();
        let mut power = vec![1; n];
        for i in 1..n {
            power[i] = (power[i - 1] * 2) % MOD;
        }

        let mut left = 0usize;
        let mut right = n - 1;
        let mut result = 0;

        while left <= right {
            if (nums[left] + nums[right]) <= target {
                result = (result + power[right - left]) % MOD;
                left += 1;
            } else {
                if right > 0 {
                    right -= 1;
                } else {
                    break;
                }
            }
        }

        result
    }
}

struct Input {
    nums: Vec<i32>,
    target: i32,
}

fn main() {
    let inputs = vec![
        Input {
            nums: vec![3, 5, 6, 7],
            target: 9,
        },
        Input {
            nums: vec![3, 3, 6, 8],
            target: 10,
        },
        Input {
            nums: vec![2, 3, 3, 4, 6, 7],
            target: 12,
        },
        Input {
            nums: vec![1],
            target: 1,
        },
    ];

    for input in inputs {
        let result = Solution::num_subseq(input.nums, input.target);
        println!("{:?}", result);
    }
}

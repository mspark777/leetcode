struct Solution {}

impl Solution {
    pub fn maximum_length(nums: Vec<i32>, k: i32) -> i32 {
        let k = k as usize;
        let mut dp = vec![vec![0; k]; k];
        let mut result = 0;

        for num in nums.iter().cloned() {
            let m = (num % k as i32) as usize;
            for prev in 0..k {
                dp[prev][m] = dp[m][prev] + 1;
                result = result.max(dp[prev][m]);
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
    let inputs = vec![
        Input {
            nums: [1, 2, 3, 4, 5].to_vec(),
            k: 2,
        },
        Input {
            nums: [1, 4, 2, 3, 1, 4].to_vec(),
            k: 3,
        },
    ];

    for input in inputs {
        let result = Solution::maximum_length(input.nums, input.k);
        println!("{:?}", result);
    }
}

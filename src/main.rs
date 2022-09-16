struct Solution {}
impl Solution {
    pub fn maximum_score(nums: Vec<i32>, multipliers: Vec<i32>) -> i32 {
        let n = nums.len();
        let m = multipliers.len();

        let mut dp = vec![0; m + 1];
        for op in (0..m).rev() {
            let row = dp.clone();

            for left in (0..=op).rev() {
                let n0 = multipliers[op] * nums[left] + row[left + 1];
                let n1 = multipliers[op] * nums[n - 1 - (op - left)] + row[left];
                dp[left] = n0.max(n1);
            }
        }

        dp[0]
    }
}

struct Input {
    nums: Vec<i32>,
    multipliers: Vec<i32>,
}

fn main() {
    let inputs: Vec<Input> = vec![
        Input {
            nums: vec![1, 2, 3],
            multipliers: vec![3, 2, 1],
        },
        Input {
            nums: vec![-5, -3, -3, -2, 7, 1],
            multipliers: vec![-10, -5, 3, 4, 6],
        },
    ];

    for Input { nums, multipliers } in inputs.into_iter() {
        let result = Solution::maximum_score(nums, multipliers);
        println!("{result:?}");
    }
}

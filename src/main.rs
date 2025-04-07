struct Solution {}

impl Solution {
    pub fn can_partition(nums: Vec<i32>) -> bool {
        let sum = nums.iter().sum::<i32>();
        let sum = sum as usize;
        if (sum & 1) == 1 {
            return false;
        }

        let target = sum / 2;
        let mut dp = vec![false; target + 1];
        dp[0] = true;

        for num in nums.iter().cloned() {
            let num = num as usize;
            for curr_sum in (num..=target).rev() {
                dp[curr_sum] = dp[curr_sum] || dp[curr_sum - num];
                if dp[target] {
                    return true;
                }
            }
        }

        return dp[target];
    }
}

struct Input {
    nums: Vec<i32>,
}

fn main() {
    let inputs = vec![
        Input {
            nums: vec![1, 5, 11, 5],
        },
        Input {
            nums: vec![1, 2, 3, 5],
        },
    ];

    for input in inputs {
        let result = Solution::can_partition(input.nums);
        println!("{result:?}");
    }
}

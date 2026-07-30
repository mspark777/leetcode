struct Solution;

impl Solution {
    pub fn delete_and_earn(nums: Vec<i32>) -> i32 {
        let mut sum = vec![0; 10001];
        let mut dp = vec![0; 10001];

        for num in nums {
            sum[num as usize] += num;
        }

        dp[1] = sum[1];

        for i in 2..10001 {
            dp[i] = i32::max(sum[i] + dp[i - 2], dp[i - 1])
        }

        dp[10001 - 1]
    }
}

struct Input {
    nums: Vec<i32>,
}

fn main() {
    let inputs = [
        Input {
            nums: vec![3, 4, 2],
        },
        Input {
            nums: vec![2, 2, 3, 3, 3, 4],
        },
    ];

    for input in inputs.into_iter() {
        let result = Solution::delete_and_earn(input.nums);
        println!("{:?}", result);
    }
}

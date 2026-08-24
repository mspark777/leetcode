struct Solution;

impl Solution {
    pub fn num_subarrays_with_sum(nums: Vec<i32>, goal: i32) -> i32 {
        let n = nums.len();
        let mut result = 0;
        let mut sum = 0;
        let mut cnt = vec![0; n + 1];
        cnt[0] = 1;
        for x in nums {
            if sum + x - goal >= 0 {
                result += cnt[(sum + x - goal) as usize];
            }
            sum += x;
            cnt[sum as usize] += 1;
        }
        result
    }
}

struct Input {
    nums: Vec<i32>,
    goal: i32,
}

fn main() {
    let inputs = [Input {
        nums: [1, 0, 1, 0, 1].to_vec(),
        goal: 2,
    }];

    for input in inputs {
        let result = Solution::num_subarrays_with_sum(input.nums, input.goal);
        println!("{:?}", result);
    }
}

struct Solution {}
impl Solution {
    pub fn three_sum_closest(nums: Vec<i32>, target: i32) -> i32 {
        let mut nums = nums;
        nums.sort_unstable();

        let mut result = 0;
        let mut diffresult = i32::max_value();

        for (i, ni) in nums.iter().enumerate() {
            let mut j = i + 1;
            let mut k = nums.len() - 1;

            while j < k {
                let nj = nums[j];
                let nk = nums[k];
                let sum = ni + nj + nk;
                let diffsum = (target - sum).abs();

                if diffsum < diffresult {
                    result = sum;
                    diffresult = diffsum;
                }

                if sum < target {
                    j += 1;
                } else if sum > target {
                    k -= 1;
                } else {
                    return sum;
                }
            }
        }

        return result;
    }
}

struct Input {
    nums: Vec<i32>,
    target: i32,
}

fn main() {
    let inputs = [
        Input {
            nums: vec![-1, 2, 1, -4],
            target: 1,
        },
        Input {
            nums: vec![0, 0, 0],
            target: 1,
        },
    ];

    for Input { nums, target } in inputs {
        let result = Solution::three_sum_closest(nums, target);
        println!("{result}");
    }
}

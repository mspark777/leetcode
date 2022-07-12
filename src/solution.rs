pub struct Solution {}

impl Solution {
    pub fn makesquare(matchsticks: Vec<i32>) -> bool {
        let sum = matchsticks.iter().fold(0, |acc, cur| acc + cur);
        if (sum % 4) != 0 {
            return false;
        }

        let mut nums = matchsticks;
        nums.sort_unstable_by(|a, b| b.cmp(a));
        Self::dfs(&nums, &mut [0, 0, 0, 0], 0, sum / 4)
    }

    fn dfs(nums: &Vec<i32>, sums: &mut [i32; 4], index: usize, target: i32) -> bool {
        if index >= nums.len() {
            return sums.iter().skip(1).all(|v| *v == target);
        }

        for i in 0..4 {
            if (sums[i] + nums[index]) > target {
                continue;
            }

            sums[i] += nums[index];
            if Self::dfs(nums, sums, index + 1, target) {
                return true;
            }
            sums[i] -= nums[index];
        }

        false
    }
}

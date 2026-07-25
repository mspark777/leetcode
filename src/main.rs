struct Solution;

impl Solution {
    pub fn can_partition_k_subsets(nums: Vec<i32>, k: i32) -> bool {
        if k == 1 {
            return true;
        }

        let target = nums.iter().sum::<i32>();
        match target % k {
            0 => Self::dfs(0, target / k, k as usize, &nums, &mut [0; 16]),
            _ => false,
        }
    }

    fn dfs(i: usize, target: i32, k: usize, nums: &[i32], sums: &mut [i32]) -> bool {
        if i == nums.len() {
            return true;
        }

        for j in 0..k {
            if nums[i] + sums[j] <= target {
                sums[j] += nums[i];
                if Self::dfs(i + 1, target, k, nums, sums) {
                    return true;
                }
                sums[j] -= nums[i];
            }

            if sums[j] == 0 {
                break;
            }
        }

        false
    }
}

struct Input {
    nums: Vec<i32>,
    k: i32,
}

fn main() {
    let inputs = [Input {
        nums: [4, 3, 2, 3, 5, 2, 1].to_vec(),
        k: 4,
    }];

    for input in inputs.into_iter() {
        let result = Solution::can_partition_k_subsets(input.nums, input.k);
        println!("{:?}", result);
    }
}

use std::collections::VecDeque;

struct Solution {}

impl Solution {
    pub fn shortest_subarray(nums: Vec<i32>, k: i32) -> i32 {
        let nums_len = nums.len();
        let mut prefix_sums = vec![0i64; nums_len + 1];

        for i in 1..=nums_len {
            prefix_sums[i] = prefix_sums[i - 1] + (nums[i - 1] as i64);
        }

        let mut result = i32::MAX;
        let mut candidate_indices = VecDeque::<usize>::with_capacity(nums_len + 1);
        for i in 0..=nums_len {
            while let Some(&candidate_index) = candidate_indices.front() {
                let prefix_sum = prefix_sums[i] - prefix_sums[candidate_index];
                if prefix_sum >= (k as i64) {
                    result = result.min(Self::sub_usize(i, candidate_index));
                    candidate_indices.pop_front();
                } else {
                    break;
                }
            }

            while let Some(&candidate_index) = candidate_indices.back() {
                if prefix_sums[i] <= prefix_sums[candidate_index] {
                    candidate_indices.pop_back();
                } else {
                    break;
                }
            }

            candidate_indices.push_back(i);
        }

        return if result == i32::MAX { -1 } else { result };
    }

    fn sub_usize(left: usize, right: usize) -> i32 {
        return if left < right {
            -Self::sub_usize(right, left)
        } else {
            (left - right) as i32
        };
    }
}

struct Input {
    nums: Vec<i32>,
    k: i32,
}

fn main() {
    let inputs = vec![
        Input {
            nums: vec![1],
            k: 1,
        },
        Input {
            nums: vec![1, 2],
            k: 4,
        },
        Input {
            nums: vec![2, -1, 2],
            k: 3,
        },
    ];

    for input in inputs {
        let result = Solution::shortest_subarray(input.nums, input.k);
        println!("{result:?}");
    }
}

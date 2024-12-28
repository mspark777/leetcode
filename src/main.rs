struct Solution {}

impl Solution {
    pub fn max_sum_of_three_subarrays(nums: Vec<i32>, k: i32) -> Vec<i32> {
        let mut best_single_start = 0;
        let mut best_double_start = vec![0, k];
        let mut result = vec![0, k, k * 2];

        let k = k as usize;

        let mut current_window_sum_single = 0;
        for &num in nums.iter().take(k) {
            current_window_sum_single += num;
        }

        let mut current_window_sum_double = 0;
        for &num in nums.iter().skip(k).take(k) {
            current_window_sum_double += num;
        }

        let mut current_window_sum_triple = 0;
        for &num in nums.iter().skip(k * 2).take(k) {
            current_window_sum_triple += num;
        }

        let mut best_single_sum = current_window_sum_single;
        let mut best_double_sum = current_window_sum_single + current_window_sum_double;
        let mut best_triple_sum =
            current_window_sum_single + current_window_sum_double + current_window_sum_triple;

        let mut single_start_index = 1usize;
        let mut double_start_index = (k + 1) as usize;
        let mut triple_start_index = (k * 2 + 1) as usize;

        while triple_start_index <= (nums.len() - (k as usize)) {
            current_window_sum_single = current_window_sum_single - nums[single_start_index - 1]
                + nums[single_start_index + k - 1];

            current_window_sum_double = current_window_sum_double - nums[double_start_index - 1]
                + nums[double_start_index + k - 1];

            current_window_sum_triple = current_window_sum_triple - nums[triple_start_index - 1]
                + nums[triple_start_index + k - 1];

            if current_window_sum_single > best_single_sum {
                best_single_start = single_start_index;
                best_single_sum = current_window_sum_single;
            }

            if (current_window_sum_double + best_single_sum) > best_double_sum {
                best_double_start[0] = best_single_start as i32;
                best_double_start[1] = double_start_index as i32;
                best_double_sum = current_window_sum_double + best_single_sum;
            }

            if (current_window_sum_triple + best_double_sum) > best_triple_sum {
                result[0] = best_double_start[0];
                result[1] = best_double_start[1];
                result[2] = triple_start_index as i32;
                best_triple_sum = current_window_sum_triple + best_double_sum;
            }

            single_start_index += 1;
            double_start_index += 1;
            triple_start_index += 1;
        }

        return result;
    }
}

struct Input {
    nums: Vec<i32>,
    k: i32,
}

fn main() {
    let inputs = vec![
        Input {
            nums: vec![1, 2, 1, 2, 6, 7, 5, 1],
            k: 2,
        },
        Input {
            nums: vec![1, 2, 1, 2, 1, 2, 1, 2, 1],
            k: 2,
        },
    ];

    for input in inputs {
        let result = Solution::max_sum_of_three_subarrays(input.nums, input.k);
        println!("{result:?}");
    }
}

struct Solution {}

impl Solution {
    pub fn longest_nice_subarray(nums: Vec<i32>) -> i32 {
        let mut used_bits = 0;
        let mut window_start = 0usize;
        let mut max_length = 0usize;

        for window_end in 0..nums.len() {
            while (used_bits & nums[window_end]) != 0 {
                used_bits ^= nums[window_start];
                window_start += 1;
            }

            used_bits |= nums[window_end];
            max_length = max_length.max(window_end - window_start + 1);
        }

        return max_length as i32;
    }
}

struct Input {
    nums: Vec<i32>,
}

fn main() {
    let inputs = vec![
        Input {
            nums: vec![1, 3, 8, 48, 10],
        },
        Input {
            nums: vec![3, 1, 5, 11, 13],
        },
    ];

    for input in inputs {
        let result = Solution::longest_nice_subarray(input.nums);
        println!("{result:?}");
    }
}

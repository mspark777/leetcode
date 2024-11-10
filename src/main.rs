struct Solution {}

impl Solution {
    pub fn minimum_subarray_length(nums: Vec<i32>, k: i32) -> i32 {
        let mut min_length = usize::MAX;
        let mut window_start = 0usize;
        let mut window_end = 0usize;
        let mut bit_counts = vec![0; 32];

        while window_end < nums.len() {
            Self::update_bit_counts(&mut bit_counts, nums[window_end], 1);

            while window_start <= window_end
                && Self::convert_bit_counts_to_number(&mut bit_counts) >= k
            {
                min_length = min_length.min(window_end - window_start + 1);
                Self::update_bit_counts(&mut bit_counts, nums[window_start], -1);
                window_start += 1;
            }
            window_end += 1;
        }

        return match min_length {
            usize::MAX => -1,
            _ => min_length as i32,
        };
    }

    fn update_bit_counts(bit_counts: &mut Vec<i32>, num: i32, delta: i32) {
        for bit_position in 0..32 {
            let lsb = (num >> bit_position) & 1;
            if lsb == 1 {
                bit_counts[bit_position] += delta;
            }
        }
    }

    fn convert_bit_counts_to_number(bit_counts: &mut Vec<i32>) -> i32 {
        let mut result = 0;
        for bit_position in 0..32 {
            if bit_counts[bit_position] != 0 {
                result |= 1 << bit_position;
            }
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
            nums: vec![1, 2, 3],
            k: 2,
        },
        Input {
            nums: vec![2, 1, 8],
            k: 10,
        },
        Input {
            nums: vec![1, 2],
            k: 0,
        },
    ];

    for input in inputs {
        let result = Solution::minimum_subarray_length(input.nums, input.k);
        println!("{result}");
    }
}

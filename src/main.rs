struct Solution {}

impl Solution {
    pub fn longest_alternating_subarray(nums: Vec<i32>, threshold: i32) -> i32 {
        let mut result = 0;
        let mut count = 0;
        let mut i = 0usize;

        while i < nums.len() {
            let num = nums[i];
            if ((num & 1) == (count & 1)) && (num <= threshold) {
                count += 1;
                result = result.max(count);
            } else if count > 0 {
                count = 0;
                i -= 1;
            }

            i += 1;
        }

        result
    }
}

struct Input {
    nums: Vec<i32>,
    threshold: i32,
}

fn main() {
    let inputs = [
        Input {
            nums: [3, 2, 5, 4].to_vec(),
            threshold: 5,
        },
        Input {
            nums: [1, 2].to_vec(),
            threshold: 2,
        },
        Input {
            nums: [2, 3, 4, 5].to_vec(),
            threshold: 4,
        },
    ];

    for input in inputs {
        let result = Solution::longest_alternating_subarray(input.nums, input.threshold);
        println!("{:?}", result);
    }
}

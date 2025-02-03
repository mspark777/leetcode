struct Solution {}

impl Solution {
    pub fn longest_monotonic_subarray(nums: Vec<i32>) -> i32 {
        let mut inc_length = 1;
        let mut dec_length = 1;
        let mut result = 1;

        for (curr, next) in nums.windows(2).map(|w| (w[0], w[1])) {
            if curr < next {
                inc_length += 1;
                dec_length = 1;
            } else if curr > next {
                dec_length += 1;
                inc_length = 1;
            } else {
                inc_length = 1;
                dec_length = 1;
            }

            result = result.max(inc_length.max(dec_length));
        }

        return result;
    }
}

struct Input {
    nums: Vec<i32>,
}

fn main() {
    let inputs = vec![
        Input {
            nums: vec![1, 4, 3, 3, 2],
        },
        Input {
            nums: vec![3, 3, 3, 3],
        },
        Input {
            nums: vec![3, 2, 1],
        },
    ];

    for input in inputs {
        let result = Solution::longest_monotonic_subarray(input.nums);
        println!("{result:?}");
    }
}

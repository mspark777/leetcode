struct Solution;

impl Solution {
    pub fn smallest_range_ii(mut nums: Vec<i32>, k: i32) -> i32 {
        nums.sort_unstable();
        let n = nums.len();

        let a = nums.first().unwrap() + k;
        let b = nums.last().unwrap() - k;

        let mut result = nums[n - 1] - nums[0];
        for i in 0..(n - 1) {
            result = result.min(b.max(nums[i] + k) - a.min(nums[i + 1] - k));
        }

        result
    }
}

struct Input {
    arr: Vec<i32>,
}

fn main() {
    let inputs = [Input { arr: [0].to_vec() }];

    for input in inputs {
        let result = Solution::subarray_bitwise_o_rs(input.arr);
        println!("{:?}", result);
    }
}

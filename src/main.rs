struct Solution;

impl Solution {
    pub fn longest_ones(nums: Vec<i32>, k: i32) -> i32 {
        let n = nums.len();
        let mut k = k;
        let mut lo = 0usize;
        let mut hi = 0usize;
        while hi < n {
            if nums[hi] == 0 {
                k -= 1;
            }
            if k < 0 {
                if nums[lo] == 0 {
                    k += 1;
                }
                lo += 1;
            }
            hi += 1;
        }
        (hi - lo) as i32
    }
}

struct Input {
    start_value: i32,
    target: i32,
}

fn main() {
    let inputs = [Input {
        start_value: 2,
        target: 3,
    }];

    for input in inputs {
        let result = Solution::broken_calc(input.start_value, input.target);
        println!("{:?}", result);
    }
}

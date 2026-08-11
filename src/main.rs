struct Solution;

impl Solution {
    pub fn num_subarray_bounded_max(nums: Vec<i32>, left: i32, right: i32) -> i32 {
        Self::count(&nums, right) - Self::count(&nums, left - 1)
    }

    fn count(nums: &[i32], bound: i32) -> i32 {
        let mut count = 0;
        let mut length = 0;

        for &num in nums {
            if num <= bound {
                length += 1;
                count += length;
            } else {
                length = 0;
            }
        }

        count
    }
}

struct Input {
    nums: Vec<i32>,
    left: i32,
    right: i32,
}

fn main() {
    let inputs = [Input {
        nums: [2, 1, 4, 3].to_vec(),
        left: 2,
        right: 3,
    }];

    for input in inputs {
        let result = Solution::num_subarray_bounded_max(input.nums, input.left, input.right);
        println!("{:?}", result);
    }
}

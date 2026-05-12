struct Solution;

impl Solution {
    pub fn number_of_arithmetic_slices(nums: Vec<i32>) -> i32 {
        let n = nums.len();
        let mut result = 0;
        let mut curr = 0;

        for i in 2..n {
            let n0 = nums[i - 2];
            let n1 = nums[i - 1];
            let n2 = nums[i];

            if (n2 - n1) == (n1 - n0) {
                curr += 1;
                result += curr;
            } else {
                curr = 0;
            }
        }

        result
    }
}

struct Input {
    nums: Vec<i32>,
}

fn main() {
    let inputs = [
        Input {
            nums: [1, 2, 3, 4].to_vec(),
        },
        Input { nums: [1].to_vec() },
    ];

    for input in inputs.into_iter() {
        let result = Solution::number_of_arithmetic_slices(input.nums);
        println!("{:?}", result);
    }
}

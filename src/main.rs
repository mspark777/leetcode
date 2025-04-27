struct Solution {}

impl Solution {
    pub fn count_subarrays(nums: Vec<i32>) -> i32 {
        let mut result = 0;

        for i in 1..(nums.len() - 1) {
            let first = nums[i - 1];
            let second = nums[i];
            let third = nums[i + 1];
            let sum = (first + third) * 2;

            if second == sum {
                result += 1;
            }
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
            nums: vec![1, 2, 1, 4, 1],
        },
        Input {
            nums: vec![1, 1, 1],
        },
    ];

    for input in inputs {
        let result = Solution::count_subarrays(input.nums);
        println!("{result:?}");
    }
}

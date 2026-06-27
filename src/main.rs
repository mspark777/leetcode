struct Solution;

impl Solution {
    pub fn array_nesting(nums: Vec<i32>) -> i32 {
        let mut result = 0;
        let mut nums = nums;
        let n = nums.len();

        for i in 0..n {
            if nums[i] == i32::MAX {
                continue;
            }

            let mut start = nums[i];
            let mut count = 0;
            while nums[start as usize] != i32::MAX {
                let temp = start;
                start = nums[start as usize];

                count += 1;
                nums[temp as usize] = i32::MAX;
            }
            result = result.max(count);
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
            nums: [5, 4, 0, 3, 1, 6, 2].to_vec(),
        },
        Input {
            nums: [0, 1, 2].to_vec(),
        },
    ];

    for input in inputs.into_iter() {
        let result = Solution::array_nesting(input.nums);
        println!("{:?}", result);
    }
}

struct Solution {}
impl Solution {
    pub fn move_zeroes(nums: &mut Vec<i32>) {
        let mut last_zero = 0;

        for i in 0..nums.len() {
            if nums[i] != 0 {
                nums[last_zero] = nums[i];
                last_zero += 1;
            }
        }

        for i in last_zero..nums.len() {
            nums[i] = 0;
        }
    }
}

struct Input {
    nums: Vec<i32>,
}

fn main() {
    let inputs: Vec<Input> = vec![
        Input {
            nums: vec![0, 1, 0, 3, 12],
        },
        Input { nums: vec![0] },
    ];

    for input in inputs {
        let mut nums = input.nums;
        Solution::move_zeroes(&mut nums);
        println!("{:?}", nums);
    }
}

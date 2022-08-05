struct Solution {}
impl Solution {
    pub fn combination_sum4(nums: Vec<i32>, target: i32) -> i32 {
        let dp_len = (target + 1) as usize;
        let mut result = vec![0; dp_len];
        result[0] = 1;

        for i in 1..dp_len {
            for num in nums.iter() {
                let n = *num as usize;
                if i >= n {
                    result[i] += result[i - n];
                }
            }
        }

        result[dp_len - 1]
    }
}

struct Input {
    nums: Vec<i32>,
    target: i32,
}

fn main() {
    let inputs: Vec<Input> = vec![
        Input {
            nums: vec![1, 2, 3],
            target: 4,
        },
        Input {
            nums: vec![9],
            target: 3,
        },
    ];

    for input in inputs {
        let nums = input.nums;
        let target = input.target;
        let result = Solution::combination_sum4(nums, target);
        println!("{:?}", result);
    }
}

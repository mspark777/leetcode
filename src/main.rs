struct Solution;

impl Solution {
    pub fn max_rotate_function(nums: Vec<i32>) -> i32 {
        let n = nums.len() as i32;
        let mut f = 0;
        let mut num_sum = 0;
        for (i, num) in nums.iter().copied().enumerate() {
            num_sum += num;
            f += (i as i32) * num;
        }

        let mut result = f;
        for num in nums.iter().copied().rev() {
            f += num_sum - n * num;
            result = result.max(f);
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
            nums: [4, 3, 2, 6].to_vec(),
        },
        Input {
            nums: [100].to_vec(),
        },
    ];

    for input in inputs.into_iter() {
        let result = Solution::max_rotate_function(input.nums);
        println!("{:?}", result);
    }
}

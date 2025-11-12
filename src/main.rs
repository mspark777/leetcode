struct Solution {}

impl Solution {
    pub fn left_right_difference(nums: Vec<i32>) -> Vec<i32> {
        let mut result = vec![0; nums.len()];
        let mut sum = 0;
        for (i, num) in nums.iter().cloned().enumerate() {
            result[i] = sum;
            sum += num;
        }

        sum = 0;
        for (i, num) in nums.iter().cloned().enumerate().rev() {
            result[i] = (sum - result[i]).abs();
            sum += num;
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
            nums: [10, 4, 8, 3].to_vec(),
        },
        Input { nums: [1].to_vec() },
    ];

    for input in inputs {
        let result = Solution::left_right_difference(input.nums);
        println!("{:?}", result);
    }
}

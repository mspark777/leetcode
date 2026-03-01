struct Solution;

impl Solution {
    pub fn construct_transformed_array(nums: Vec<i32>) -> Vec<i32> {
        let n = nums.len() as i32;
        let mut result = vec![0; n as usize];

        for (i, num) in nums.iter().copied().enumerate() {
            let i = i as i32;
            let j = (((i + num) % n) + n) % n;
            result[i as usize] = nums[j as usize];
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
            nums: [3, -2, 1, 1].to_vec(),
        },
        Input {
            nums: [-1, 4, -1].to_vec(),
        },
    ];

    for input in inputs {
        let result = Solution::construct_transformed_array(input.nums);
        println!("{:?}", result);
    }
}

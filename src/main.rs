struct Solution {}

impl Solution {
    pub fn build_array(nums: Vec<i32>) -> Vec<i32> {
        let mut result = vec![0; nums.len()];

        for (i, num) in nums.iter().cloned().enumerate() {
            let j = num as usize;
            result[i] = nums[j];
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
            nums: vec![0, 2, 1, 5, 3, 4],
        },
        Input {
            nums: vec![5, 0, 1, 2, 3, 4],
        },
    ];

    for input in inputs {
        let result = Solution::build_array(input.nums);
        println!("{result:?}");
    }
}

struct Solution {}

impl Solution {
    pub fn create_target_array(nums: Vec<i32>, index: Vec<i32>) -> Vec<i32> {
        let mut result = vec![0; nums.len()];
        for (i, num) in index.iter().cloned().zip(nums.iter().cloned()) {
            let i = i as usize;
            for j in ((i + 1)..nums.len()).rev() {
                result[j] = result[j - 1];
            }

            result[i] = num;
        }

        result
    }
}

struct Input {
    nums: Vec<i32>,
    index: Vec<i32>,
}

fn main() {
    let inputs = [
        Input {
            nums: [0, 1, 2, 3, 4].to_vec(),
            index: [0, 1, 2, 2, 1].to_vec(),
        },
        Input {
            nums: [1, 2, 3, 4, 0].to_vec(),
            index: [0, 1, 2, 3, 0].to_vec(),
        },
        Input {
            nums: [1].to_vec(),
            index: [0].to_vec(),
        },
    ];

    for input in inputs {
        let result = Solution::create_target_array(input.nums, input.index);
        println!("{:?}", result);
    }
}

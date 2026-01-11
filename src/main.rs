struct Solution;

impl Solution {
    pub fn number_game(nums: Vec<i32>) -> Vec<i32> {
        let mut result = nums.clone();
        result.sort();

        for i in (1..result.len()).step_by(2) {
            let left = result[i - 1];
            let right = result[i];

            result[i - 1] = right;
            result[i] = left;
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
            nums: [5, 4, 2, 3].to_vec(),
        },
        Input {
            nums: [2, 5].to_vec(),
        },
    ];

    for input in inputs {
        let result = Solution::number_game(input.nums);
        println!("{:?}", result);
    }
}

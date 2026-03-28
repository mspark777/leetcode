struct Solution;

impl Solution {
    pub fn min_moves(nums: Vec<i32>) -> i32 {
        let mut max_num = 0;
        let mut sum = 0;
        let n = nums.len() as i32;

        for num in nums {
            sum += num;
            max_num = max_num.max(num);
        }

        (max_num * n) - sum
    }
}

struct Input {
    nums: Vec<i32>,
}

fn main() {
    let inputs = [
        Input {
            nums: [2, 1, 3].to_vec(),
        },
        Input {
            nums: [4, 4, 5].to_vec(),
        },
    ];

    for input in inputs {
        let result = Solution::min_moves(input.nums);
        println!("{:?}", result);
    }
}

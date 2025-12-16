struct Solution {}

impl Solution {
    pub fn sum_of_squares(nums: Vec<i32>) -> i32 {
        let n = nums.len();
        nums.iter()
            .copied()
            .enumerate()
            .filter(|&(i, _)| n.is_multiple_of(i + 1))
            .map(|(_, num)| num * num)
            .sum()
    }
}

struct Input {
    nums: Vec<i32>,
}

fn main() {
    let inputs = [
        Input {
            nums: [1, 2, 3, 4].to_vec(),
        },
        Input {
            nums: [2, 7, 1, 19, 18, 3].to_vec(),
        },
    ];

    for input in inputs {
        let result = Solution::sum_of_squares(input.nums);
        println!("{:?}", result);
    }
}

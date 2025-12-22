struct Solution;

impl Solution {
    pub fn count_pairs(nums: Vec<i32>, target: i32) -> i32 {
        let mut result = 0;
        for (i, l) in nums.iter().copied().enumerate() {
            for r in nums.iter().skip(i + 1).copied() {
                if (l + r) < target {
                    result += 1;
                }
            }
        }
        result
    }
}

struct Input {
    nums: Vec<i32>,
    target: i32,
}

fn main() {
    let inputs = [
        Input {
            nums: [-1, 1, 2, 3, 1].to_vec(),
            target: 2,
        },
        Input {
            nums: [-6, 2, 5, -2, -7, -1, 3].to_vec(),
            target: -2,
        },
    ];

    for input in inputs {
        let result = Solution::count_pairs(input.nums, input.target);
        println!("{:?}", result);
    }
}

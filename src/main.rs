struct Solution;

impl Solution {
    pub fn maximum_strong_pair_xor(nums: Vec<i32>) -> i32 {
        let mut result = 0;

        for x in nums.iter().copied() {
            for y in nums.iter().copied() {
                if (x - y).abs() <= x.min(y) {
                    result = result.max(x ^ y);
                }
            }
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
            nums: [1, 2, 3, 4, 5].to_vec(),
        },
        Input {
            nums: [10, 100].to_vec(),
        },
        Input {
            nums: [5, 6, 25, 30].to_vec(),
        },
    ];

    for input in inputs {
        let result = Solution::maximum_strong_pair_xor(input.nums);
        println!("{:?}", result);
    }
}

struct Solution;

impl Solution {
    pub fn min_absolute_difference(nums: Vec<i32>) -> i32 {
        const INVALID: i32 = 101;
        let mut left = INVALID;
        let mut right = INVALID;
        let mut difference = INVALID;

        for (i, num) in nums.into_iter().enumerate() {
            if num == 1 {
                left = i as i32;
            } else if num == 2 {
                right = i as i32;
            }

            if (left != INVALID) && (right != INVALID) {
                difference = (left - right).abs().min(difference);
            }
        }

        match difference {
            INVALID => -1,
            _ => difference,
        }
    }
}

struct Input {
    nums: Vec<i32>,
}

fn main() {
    let inputs = [
        Input {
            nums: [1, 0, 0, 2, 0, 1].to_vec(),
        },
        Input {
            nums: [1, 0, 1, 0].to_vec(),
        },
    ];

    for input in inputs {
        let result = Solution::min_absolute_difference(input.nums);
        println!("{:?}", result);
    }
}

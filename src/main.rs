struct Solution;

impl Solution {
    pub fn has_trailing_zeros(nums: Vec<i32>) -> bool {
        let mut count = 0;
        for num in nums {
            if num & 1 == 0 {
                count += 1;
            }

            if count > 1 {
                break;
            }
        }

        count > 1
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
            nums: [2, 4, 8, 16].to_vec(),
        },
        Input {
            nums: [1, 3, 5, 7, 9].to_vec(),
        },
    ];

    for input in inputs {
        let result = Solution::has_trailing_zeros(input.nums);
        println!("{:?}", result);
    }
}

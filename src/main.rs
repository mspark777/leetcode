struct Solution {}

impl Solution {
    pub fn longest_subarray(nums: Vec<i32>) -> i32 {
        let mut max_num = 0;
        let mut result = 0;
        let mut current = 0;

        for num in nums.iter().cloned() {
            if max_num < num {
                max_num = num;
                result = 0;
                current = 0;
            }

            if max_num == num {
                current += 1;
            } else {
                current = 0;
            }

            result = result.max(current);
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
            nums: [1, 2, 3, 3, 2, 2].to_vec(),
        },
        Input {
            nums: [1, 2, 3, 4].to_vec(),
        },
    ];

    for input in inputs {
        let result = Solution::longest_subarray(input.nums);
        println!("{:?}", result);
    }
}

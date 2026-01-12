struct Solution;

impl Solution {
    pub fn missing_integer(nums: Vec<i32>) -> i32 {
        let mut result = nums[0];

        if nums.len() == 1 {
            return result + 1;
        }

        let n = nums.len();
        let mut nums = nums;
        for i in 1..n {
            if (nums[i - 1] + 1) == nums[i] {
                result += nums[i];
            } else {
                nums.sort();
                for num in nums.iter().copied() {
                    if num == result {
                        result += 1;
                    }
                }
                break;
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
            nums: [1, 2, 3, 2, 5].to_vec(),
        },
        Input {
            nums: [3, 4, 5, 1, 12, 14, 13].to_vec(),
        },
    ];

    for input in inputs {
        let result = Solution::missing_integer(input.nums);
        println!("{:?}", result);
    }
}

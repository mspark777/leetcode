struct Solution;

impl Solution {
    pub fn count_valid_selections(nums: Vec<i32>) -> i32 {
        let mut result = 0;
        let mut left = 0;
        let mut right = nums.iter().sum::<i32>();

        for num in nums {
            if num == 0 {
                if (0..2).contains(&(left - right)) {
                    result += 1;
                }

                if (0..2).contains(&(right - left)) {
                    result += 1;
                }
            } else {
                left += num;
                right -= num;
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
            nums: [1, 0, 2, 0, 3].to_vec(),
        },
        Input {
            nums: [2, 3, 4, 0, 4, 1, 0].to_vec(),
        },
    ];

    for input in inputs {
        let result = Solution::count_valid_selections(input.nums);
        println!("{:?}", result);
    }
}

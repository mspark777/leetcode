struct Solution {}

impl Solution {
    pub fn minimum_size(nums: Vec<i32>, max_operations: i32) -> i32 {
        let mut left = 1;
        let mut right = nums[0];

        for &num in nums.iter() {
            right = right.max(num);
        }

        while left < right {
            let middle = (left + right) / 2;
            if Self::is_possible(middle, &nums, max_operations) {
                right = middle;
            } else {
                left = middle + 1;
            }
        }

        return left;
    }

    fn is_possible(max_balls_in_bag: i32, nums: &Vec<i32>, max_operaions: i32) -> bool {
        let mut total_operations = 0;

        for &num in nums.iter() {
            let n = num as f64;
            let mbb = max_balls_in_bag as f64;
            let operations = ((n / mbb).ceil() as i32) - 1;
            total_operations += operations;

            if total_operations > max_operaions {
                return false;
            }
        }

        return true;
    }
}

struct Input {
    nums: Vec<i32>,
    max_operations: i32,
}

fn main() {
    let inputs = vec![
        Input {
            nums: vec![9],
            max_operations: 2,
        },
        Input {
            nums: vec![2, 4, 8, 2],
            max_operations: 4,
        },
    ];

    for input in inputs {
        let result = Solution::minimum_size(input.nums, input.max_operations);
        println!("{result}");
    }
}

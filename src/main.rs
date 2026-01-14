struct Solution;

impl Solution {
    pub fn minimum_cost(nums: Vec<i32>) -> i32 {
        let first = nums[0];
        let mut second = 51;
        let mut third = 51;
        for num in nums.iter().skip(1).copied() {
            if num < second {
                third = second;
                second = num;
            } else if (num < third) && (num >= second) {
                third = num;
            }
        }

        first + second + third
    }
}

struct Input {
    nums: Vec<i32>,
}

fn main() {
    let inputs = [
        Input {
            nums: [1, 2, 3, 12].to_vec(),
        },
        Input {
            nums: [5, 4, 3].to_vec(),
        },
        Input {
            nums: [10, 3, 1, 1].to_vec(),
        },
    ];

    for input in inputs {
        let result = Solution::minimum_cost(input.nums);
        println!("{:?}", result);
    }
}

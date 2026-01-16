struct Solution;

impl Solution {
    pub fn return_to_boundary_count(nums: Vec<i32>) -> i32 {
        let mut position = 0;
        let mut result = 0;

        for num in nums {
            position += num;
            if position == 0 {
                result += 1;
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
            nums: [2, 3, -5].to_vec(),
        },
        Input {
            nums: [3, 2, -3, -4].to_vec(),
        },
    ];

    for input in inputs {
        let result = Solution::return_to_boundary_count(input.nums);
        println!("{:?}", result);
    }
}

struct Solution {}

impl Solution {
    pub fn triangle_type(nums: Vec<i32>) -> String {
        let mut nums = nums.clone();
        nums.sort_unstable();

        if (nums[0] + nums[1]) <= nums[2] {
            return "none".to_string();
        } else if nums[0] == nums[2] {
            return "equilateral".to_string();
        } else if (nums[0] == nums[1]) || (nums[1] == nums[2]) {
            return "isosceles".to_string();
        }

        return "scalene".to_string();
    }
}

struct Input {
    nums: Vec<i32>,
}

fn main() {
    let inputs = vec![
        Input {
            nums: vec![3, 3, 3],
        },
        Input {
            nums: vec![3, 4, 5],
        },
    ];

    for input in inputs {
        let result = Solution::triangle_type(input.nums);
        println!("{:?}", result);
    }
}

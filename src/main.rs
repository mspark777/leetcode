struct Solution {}

impl Solution {
    pub fn count_hill_valley(nums: Vec<i32>) -> i32 {
        let mut result = 0;
        let mut left = 0usize;

        for i in 1..(nums.len() - 1) {
            if nums[i] == nums[i + 1] {
                continue;
            }

            if (nums[i] > nums[left] && nums[i] > nums[i + 1])
                || (nums[i] < nums[left] && nums[i] < nums[i + 1])
            {
                result += 1;
            }
            left = i;
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
            nums: [2, 4, 1, 1, 6, 5].to_vec(),
        },
        Input {
            nums: [6, 6, 5, 5, 4, 1].to_vec(),
        },
    ];

    for input in inputs {
        let result = Solution::count_hill_valley(input.nums);
        println!("{:?}", result);
    }
}

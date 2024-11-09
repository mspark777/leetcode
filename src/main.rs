struct Solution {}

impl Solution {
    pub fn dominant_index(nums: Vec<i32>) -> i32 {
        let mut max_num = nums[0];
        let mut result = 0;

        for (i, &n) in nums.iter().skip(1).enumerate() {
            if n > max_num {
                max_num = n;
                result = i + 1;
            }
        }

        for (i, &n) in nums.iter().enumerate() {
            if i == result {
                continue;
            }

            let twice = n * 2;
            if twice > max_num {
                return -1;
            }
        }

        return result as i32;
    }
}

struct Input {
    nums: Vec<i32>,
}

fn main() {
    let inputs = vec![
        Input {
            nums: vec![3, 6, 1, 0],
        },
        Input {
            nums: vec![1, 2, 3, 4],
        },
    ];

    for input in inputs {
        let result = Solution::dominant_index(input.nums);
        println!("{result}");
    }
}

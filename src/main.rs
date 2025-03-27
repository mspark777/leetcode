struct Solution {}

impl Solution {
    pub fn minimum_index(nums: Vec<i32>) -> i32 {
        let mut x = nums[0];
        let mut count = 0;

        for num in nums.iter().cloned() {
            if num == x {
                count += 1;
            } else {
                count -= 1;
            }

            if count == 0 {
                x = num;
                count = 1;
            }
        }

        let mut x_count = 0;
        for num in nums.iter().cloned() {
            if num == x {
                x_count += 1;
            }
        }

        count = 0;

        for (i, num) in nums.iter().cloned().enumerate() {
            if num == x {
                count += 1;
            }

            let remaining_count = x_count - count;
            let left = ((i + 1) / 2) as i32;
            let right = ((nums.len() - i - 1) / 2) as i32;
            if (count > left) && remaining_count > right {
                return i as i32;
            }
        }

        return -1;
    }
}

struct Input {
    nums: Vec<i32>,
}

fn main() {
    let inputs = vec![
        Input {
            nums: vec![1, 2, 2, 2],
        },
        Input {
            nums: vec![2, 1, 3, 1, 1, 1, 7, 1, 2, 1],
        },
        Input {
            nums: vec![3, 3, 3, 3, 7, 2, 2],
        },
    ];

    for input in inputs {
        let result = Solution::minimum_index(input.nums);
        println!("{result:?}");
    }
}

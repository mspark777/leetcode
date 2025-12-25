struct Solution;

impl Solution {
    pub fn minimum_right_shifts(nums: Vec<i32>) -> i32 {
        let n = nums.len();
        let mut count = 0;
        let mut point = 0usize;
        let mut idx = 0usize;

        for i in 1..n {
            if nums[i] < nums[i - 1] {
                idx = i;
                point = n - i;
                if count == 1 {
                    return -1;
                } else {
                    count += 1;
                }
            }
        }

        if count == 0 {
            return 0;
        }

        if (nums[0] < nums[n - 1]) || (nums[idx - 1] < nums[n - 1]) {
            return -1;
        }

        point as i32
    }
}

struct Input {
    nums: Vec<i32>,
}

fn main() {
    let inputs = [
        Input {
            nums: [3, 4, 5, 1, 2].to_vec(),
        },
        Input {
            nums: [1, 3, 5].to_vec(),
        },
        Input {
            nums: [2, 1, 4].to_vec(),
        },
    ];

    for input in inputs {
        let result = Solution::minimum_right_shifts(input.nums);
        println!("{:?}", result);
    }
}

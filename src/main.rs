struct Solution {}

impl Solution {
    pub fn continuous_subarrays(nums: Vec<i32>) -> i64 {
        let mut left = 0i64;
        let mut right = 0i64;
        let mut result = 0i64;
        let mut window_len = 0i64;
        let mut cur_min = nums[0];
        let mut cur_max = nums[0];

        while right < (nums.len() as i64) {
            // to avoid unread wran
            let mut win_len = window_len;
            cur_min = cur_min.min(nums[right as usize]);
            cur_max = cur_max.max(nums[right as usize]);
            if (cur_max - cur_min) > 2 {
                win_len = right - left;
                result += win_len * (win_len + 1) / 2;
                left = right;
                cur_min = nums[right as usize];
                cur_max = nums[right as usize];

                while left > 0 && (nums[right as usize] - nums[(left - 1) as usize]).abs() <= 2 {
                    left -= 1;
                    cur_min = cur_min.min(nums[left as usize]);
                    cur_max = cur_max.max(nums[left as usize]);
                }

                if left < right {
                    win_len = right - left;
                    result -= win_len * (win_len + 1) / 2;
                }
            }

            window_len = win_len;
            right += 1
        }

        window_len = right - left;
        result += window_len * (window_len + 1) / 2;
        return result;
    }
}

struct Input {
    nums: Vec<i32>,
}

fn main() {
    let inputs = vec![
        Input {
            nums: vec![5, 4, 2, 4],
        },
        Input {
            nums: vec![1, 2, 3],
        },
    ];

    for input in inputs {
        let result = Solution::continuous_subarrays(input.nums);
        println!("{result}");
    }
}

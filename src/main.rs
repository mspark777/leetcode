struct Solution {}

impl Solution {
    pub fn maximum_sum(nums: Vec<i32>) -> i32 {
        let mut result = -1;
        let mut digit_mapping = [0; 82];

        for num in nums.iter().cloned() {
            let mut digit_sum = 0usize;
            let mut curr_value = num;
            while curr_value > 0 {
                let curr_digit = curr_value % 10;
                digit_sum += curr_digit as usize;
                curr_value /= 10;
            }

            if digit_mapping[digit_sum] > 0 {
                result = result.max(digit_mapping[digit_sum] + num);
            }

            digit_mapping[digit_sum] = num.max(digit_mapping[digit_sum]);
        }

        return result;
    }
}

struct Input {
    nums: Vec<i32>,
}

fn main() {
    let inputs = vec![
        Input {
            nums: vec![18, 43, 36, 13, 7],
        },
        Input {
            nums: vec![10, 12, 19, 14],
        },
    ];

    for input in inputs {
        let result = Solution::maximum_sum(input.nums);
        println!("{result:?}");
    }
}

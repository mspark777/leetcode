struct Solution {}

impl Solution {
    pub fn reformat_number(number: String) -> String {
        let nums = number
            .chars()
            .filter(|&c| c.is_digit(10))
            .collect::<Vec<char>>();

        let n = nums.len();
        let mut result = Vec::<char>::with_capacity(n + (n / 3));

        let mut i = 0usize;
        while i < n {
            let remains = n - i;
            if remains > 4 {
                result.push(nums[i]);
                result.push(nums[i + 1]);
                result.push(nums[i + 2]);
                result.push('-');
                i += 3;
            } else if remains == 4 {
                result.push(nums[i]);
                result.push(nums[i + 1]);
                result.push('-');
                result.push(nums[i + 2]);
                result.push(nums[i + 3]);
            } else if remains == 3 {
                result.push(nums[i]);
                result.push(nums[i + 1]);
                result.push(nums[i + 2]);
            } else if remains == 2 {
                result.push(nums[i]);
                result.push(nums[i + 1]);
            } else {
                result.push(nums[i]);
            }

            if remains <= 4 {
                break;
            }
        }

        result.iter().collect()
    }
}

struct Input {
    number: String,
}

fn main() {
    let inputs = [
        Input {
            number: "1-23-45 6".to_string(),
        },
        Input {
            number: "123 4-567".to_string(),
        },
        Input {
            number: "123 4-5678".to_string(),
        },
    ];

    for input in inputs {
        let result = Solution::reformat_number(input.number);
        println!("{:?}", result);
    }
}

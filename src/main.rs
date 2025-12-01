struct Solution {}

impl Solution {
    pub fn remove_trailing_zeros(num: String) -> String {
        let nums = num.chars().collect::<Vec<char>>();
        let mut right = nums.len() - 1;
        for (i, n) in nums.iter().copied().enumerate().rev() {
            if n != '0' {
                right = i;
                break;
            }
        }

        nums[0..=right].iter().collect()
    }
}

struct Input {
    num: String,
}

fn main() {
    let inputs = [
        Input {
            num: "51230100".to_string(),
        },
        Input {
            num: "123".to_string(),
        },
    ];

    for input in inputs {
        let result = Solution::remove_trailing_zeros(input.num);
        println!("{:?}", result);
    }
}

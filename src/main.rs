struct Solution {}

impl Solution {
    pub fn split_num(num: i32) -> i32 {
        let mut nums: Vec<i32> = num
            .to_string()
            .chars()
            .map(|c| ((c as u8) - b'0') as i32)
            .collect();
        nums.sort_unstable();

        let mut num1 = 0;
        let mut num2 = 0;

        for i in (1..nums.len()).step_by(2) {
            num1 = num1 * 10 + nums[i - 1];
            num2 = num2 * 10 + nums[i];
        }

        if (nums.len() & 1) == 1 {
            num1 = num1 * 10 + *nums.last().unwrap();
        }

        num1 + num2
    }
}

struct Input {
    num: i32,
}

fn main() {
    let inputs = [Input { num: 4325 }, Input { num: 687 }];

    for input in inputs {
        let result = Solution::split_num(input.num);
        println!("{:?}", result);
    }
}

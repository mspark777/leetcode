struct Solution {}

impl Solution {
    pub fn minimum_sum(num: i32) -> i32 {
        let mut digits: Vec<i32> = num
            .to_string()
            .chars()
            .map(|c| c.to_digit(10).unwrap() as i32)
            .collect();
        digits.sort();

        (digits[0] + digits[1]) * 10 + digits[2] + digits[3]
    }
}

struct Input {
    num: i32,
}

fn main() {
    let inputs = [Input { num: 2932 }, Input { num: 4009 }];

    for input in inputs {
        let result = Solution::minimum_sum(input.num);
        println!("{:?}", result);
    }
}

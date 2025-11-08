struct Solution {}

impl Solution {
    pub fn count_digits(num: i32) -> i32 {
        let mut result = 0;
        let mut digits = num;

        while digits > 0 {
            let digit = digits % 10;
            digits /= 10;
            if digit == 0 {
                continue;
            } else if (num % digit) == 0 {
                result += 1;
            }
        }

        result
    }
}

struct Input {
    num: i32,
}

fn main() {
    let inputs = [Input { num: 7 }, Input { num: 121 }, Input { num: 1248 }];

    for input in inputs {
        let result = Solution::count_digits(input.num);
        println!("{:?}", result);
    }
}

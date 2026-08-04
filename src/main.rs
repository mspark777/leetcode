struct Solution;

impl Solution {
    pub fn rotated_digits(n: i32) -> i32 {
        let mut result = 0;
        for i in 1..n + 1 {
            let mut num = i;
            let mut is_valid = true;
            let mut has_changed = false;

            while num > 0 {
                let digit = num % 10;

                if (digit == 3) || (digit == 4) || (digit == 7) {
                    is_valid = false;
                    break;
                }

                if (digit == 2) || (digit == 5) || (digit == 6) || (digit == 9) {
                    has_changed = true;
                }

                num /= 10;
            }

            if is_valid && has_changed {
                result += 1;
            }
        }

        result
    }
}

struct Input {
    n: i32,
}

fn main() {
    let inputs = [Input { n: 10 }, Input { n: 1 }, Input { n: 2 }];

    for input in inputs {
        let result = Solution::rotated_digits(input.n);
        println!("{:?}", result);
    }
}

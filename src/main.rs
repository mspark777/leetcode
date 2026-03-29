struct Solution;

impl Solution {
    pub fn sum_and_multiply(n: i32) -> i64 {
        let mut power = 1i64;
        let mut n = n as i64;
        let mut x = 0i64;
        let mut sum = 0i64;

        while n > 0 {
            let digit = n % 10;
            n /= 10;

            if digit != 0 {
                sum += digit;
                x += digit * power;
                power *= 10;
            }
        }

        x * sum
    }
}

struct Input {
    n: i32,
}

fn main() {
    let inputs = [Input { n: 10203004 }, Input { n: 1000 }];

    for input in inputs {
        let result = Solution::sum_and_multiply(input.n);
        println!("{:?}", result);
    }
}

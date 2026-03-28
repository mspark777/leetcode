struct Solution;

impl Solution {
    pub fn remove_zeros(n: i64) -> i64 {
        let mut n = n;
        let mut power = 1i64;
        let mut result = 0i64;

        while n > 0 {
            let digit = n % 10;
            n /= 10;

            if digit != 0 {
                result += digit * power;
                power *= 10;
            }
        }

        result
    }
}

struct Input {
    n: i64,
}

fn main() {
    let inputs = [Input { n: 1020030 }, Input { n: 1 }];

    for input in inputs {
        let result = Solution::remove_zeros(input.n);
        println!("{:?}", result);
    }
}

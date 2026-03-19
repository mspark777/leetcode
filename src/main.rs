struct Solution;

impl Solution {
    pub fn check_divisibility(n: i32) -> bool {
        let mut sum = 0;
        let mut product = 1;
        let mut num = n;

        while num > 0 {
            let digit = num % 10;
            num /= 10;

            sum += digit;
            product *= digit;
        }

        (n % (sum + product)) == 0
    }
}

struct Input {
    n: i32,
}

fn main() {
    let inputs = [Input { n: 99 }, Input { n: 23 }];

    for input in inputs {
        let result = Solution::check_divisibility(input.n);
        println!("{:?}", result);
    }
}

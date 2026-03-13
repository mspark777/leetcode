struct Solution;

impl Solution {
    pub fn max_product(n: i32) -> i32 {
        let mut first = 0;
        let mut second = 0;
        let mut result = 0;
        let mut n = n;

        while n > 0 {
            let digit = n % 10;
            n /= 10;

            if digit > first {
                second = first;
                first = digit;
            } else if digit > second {
                second = digit;
            }

            result = result.max(first * second);
        }

        result
    }
}

struct Input {
    n: i32,
}

fn main() {
    let inputs = [Input { n: 31 }, Input { n: 22 }, Input { n: 124 }];

    for input in inputs {
        let result = Solution::max_product(input.n);
        println!("{:?}", result);
    }
}

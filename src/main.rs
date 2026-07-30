struct Solution;

impl Solution {
    pub fn monotone_increasing_digits(n: i32) -> i32 {
        let mut n = n;
        let mut digits = Vec::<i32>::new();

        while n > 0 {
            digits.push(n % 10);
            n /= 10;
        }

        for i in 1..digits.len() {
            if digits[i - 1] < digits[i] {
                digits[i] -= 1;

                for d in digits.iter_mut().take(i) {
                    *d = 9;
                }
            }
        }

        let mut result = 0;
        let mut p = 1;

        for digit in digits {
            result += digit * p;
            p *= 10;
        }

        result
    }
}

struct Input {
    n: i32,
}

fn main() {
    let inputs = [Input { n: 10 }, Input { n: 1234 }, Input { n: 332 }];

    for input in inputs.into_iter() {
        let result = Solution::monotone_increasing_digits(input.n);
        println!("{:?}", result);
    }
}

struct Solution;

impl Solution {
    pub fn integer_replacement(n: i32) -> i32 {
        let mut n = n as i64;
        let mut result = 0;

        while n > 1 {
            if n == 3 {
                n -= 1;
            } else if (n & 1) == 0 {
                n /= 2;
            } else if (n % 4) == 1 {
                n -= 1;
            } else {
                n += 1;
            }

            result += 1;
        }

        result
    }
}

struct Input {
    n: i32,
}

fn main() {
    let inputs = [Input { n: 8 }, Input { n: 7 }];

    for input in inputs.into_iter() {
        let result = Solution::integer_replacement(input.n);
        println!("{:?}", result);
    }
}

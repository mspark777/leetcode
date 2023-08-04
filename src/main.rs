mod utils;

use utils::Solution;

impl Solution {
    pub fn my_pow(x: f64, n: i32) -> f64 {
        let mut x = x;
        let mut n = n as i64;

        if n < 0 {
            n *= -1;
            x = 1.0 / x;
        }

        let mut result = 1.0;
        while n != 0 {
            if (n & 1) == 1 {
                result *= x;
                n -= 1;
            }

            x *= x;
            n /= 2;
        }

        return result;
    }
}

fn main() {
    let inputs = [
        (2.00000, 10),
        (2.10000, 3),
        (2.00000, -2),
        (1.00000, -2147483648),
    ];

    for (x, n) in inputs {
        let result = Solution::my_pow(x, n);
        println!("{result}");
    }
}

struct Solution {}

impl Solution {
    pub fn min_end(n: i32, x: i32) -> i64 {
        return Self::min_end_64(n as i64, x as i64);
    }

    fn min_end_64(mut n: i64, x: i64) -> i64 {
        let mut result = x;
        let mut mask = 1i64;

        n -= 1;
        while n > 0 {
            if (mask & x) == 0 {
                result |= (n & 1) * mask;
                n >>= 1;
            }
            mask <<= 1;
        }

        return result;
    }
}

struct Input {
    n: i32,
    x: i32,
}

fn main() {
    let inputs = vec![Input { n: 3, x: 4 }, Input { n: 2, x: 7 }];

    for input in inputs {
        let result = Solution::min_end(input.n, input.x);
        println!("{result}");
    }
}

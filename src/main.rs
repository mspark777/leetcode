struct Solution {}

impl Solution {
    const MOD: i64 = 1000000007;
    pub fn count_good_numbers(n: i64) -> i32 {
        let left = Self::quickmul(5, (n + 1) / 2);
        let right = Self::quickmul(4, n / 2);
        return ((left * right) % Self::MOD) as i32;
    }

    fn quickmul(x: i32, y: i64) -> i64 {
        let mut result = 1i64;
        let mut mul = x as i64;
        let mut y = y;

        while y > 0 {
            if (y & 1) == 1 {
                result = (result * mul) % Self::MOD;
            }

            mul = (mul * mul) % Self::MOD;
            y >>= 1;
        }

        return result;
    }
}

struct Input {
    n: i64,
}

fn main() {
    let inputs = vec![Input { n: 1 }, Input { n: 4 }, Input { n: 50 }];

    for input in inputs {
        let result = Solution::count_good_numbers(input.n);
        println!("{result:?}");
    }
}

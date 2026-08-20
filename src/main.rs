struct Solution;

impl Solution {
    pub fn prime_palindrome(n: i32) -> i32 {
        if (8..=11).contains(&n) {
            return 11;
        }

        for x in 1..100000 {
            let s = x.to_string();
            let y: i32 = format!("{}{}", s, s.chars().rev().skip(1).collect::<String>())
                .parse()
                .unwrap();
            if y >= n && Self::is_prime(y) {
                return y;
            }
        }

        -1
    }

    fn is_prime(n: i32) -> bool {
        if n < 2 || n % 2 == 0 {
            return n == 2;
        }

        let mut x = 3;
        while x * x <= n {
            if n % x == 0 {
                return false;
            }
            x += 2;
        }
        true
    }
}

struct Input {
    n: i32,
}

fn main() {
    let inputs = [Input { n: 6 }];

    for input in inputs {
        let result = Solution::prime_palindrome(input.n);
        println!("{:?}", result);
    }
}

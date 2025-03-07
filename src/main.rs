struct Solution {}

impl Solution {
    pub fn closest_primes(left: i32, right: i32) -> Vec<i32> {
        let mut prev_prime = -1;
        let mut closet_a = -1;
        let mut closet_b = -1;
        let mut min_difference = 1_000_000;

        for candidate in left..=right {
            if !Self::is_prime(candidate) {
                continue;
            }

            if prev_prime == -1 {
                prev_prime = candidate;
                continue;
            }

            let difference = candidate - prev_prime;
            if difference < min_difference {
                min_difference = difference;
                closet_a = prev_prime;
                closet_b = candidate;
            }

            if difference == 2 || difference == 1 {
                return vec![prev_prime, candidate];
            }

            prev_prime = candidate;
        }

        return if closet_a != -1 {
            vec![closet_a, closet_b]
        } else {
            vec![-1, -1]
        };
    }

    fn is_prime(n: i32) -> bool {
        if n < 2 {
            return false;
        } else if (n == 2) || (n == 3) {
            return true;
        } else if n & 1 == 0 {
            return false;
        }

        let mut divisor = 3;
        while (divisor * divisor) <= n {
            if n % divisor == 0 {
                return false;
            }
            divisor += 2;
        }

        return true;
    }
}

struct Input {
    left: i32,
    right: i32,
}

fn main() {
    let inputs = vec![
        Input {
            left: 10,
            right: 19,
        },
        Input { left: 4, right: 6 },
    ];

    for input in inputs {
        let result = Solution::closest_primes(input.left, input.right);
        println!("{result:?}");
    }
}

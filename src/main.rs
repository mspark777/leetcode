struct Solution {}

impl Solution {
    pub fn common_factors(a: i32, b: i32) -> i32 {
        let gcd = Self::gcd(a, b);
        let mut result = 0;
        for i in 1..=gcd {
            if (i * i) > gcd {
                break;
            }

            if gcd % i == 0 {
                result += 1;
                if i != gcd / i {
                    result += 1;
                }
            }
        }

        result
    }

    fn gcd(mut a: i32, mut b: i32) -> i32 {
        while b != 0 {
            let temp = b;
            b = a % b;
            a = temp;
        }

        a
    }
}

struct Input {
    a: i32,
    b: i32,
}

fn main() {
    let inputs = [Input { a: 12, b: 6 }, Input { a: 25, b: 30 }];

    for input in inputs {
        let result = Solution::common_factors(input.a, input.b);
        println!("{:?}", result);
    }
}

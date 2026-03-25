struct Solution;

impl Solution {
    pub fn decimal_representation(n: i32) -> Vec<i32> {
        let mut result = Vec::<i32>::new();
        let mut n = n;
        let mut power = 1;
        while n > 0 {
            let digit = n % 10;
            n /= 10;

            if digit != 0 {
                result.push(digit * power);
            }
            power *= 10;
        }

        result.reverse();
        result
    }
}

struct Input {
    n: i32,
}

fn main() {
    let inputs = [Input { n: 537 }, Input { n: 102 }, Input { n: 6 }];

    for input in inputs {
        let result = Solution::decimal_representation(input.n);
        println!("{:?}", result);
    }
}

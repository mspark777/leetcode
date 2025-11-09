struct Solution {}

impl Solution {
    pub fn alternate_digit_sum(n: i32) -> i32 {
        let mut digits = n;
        let mut result = 0;
        while digits > 0 {
            result = (digits % 10) - result;
            digits /= 10;
        }

        result
    }
}

struct Input {
    n: i32,
}

fn main() {
    let inputs = [Input { n: 521 }, Input { n: 111 }, Input { n: 886996 }];

    for input in inputs {
        let result = Solution::alternate_digit_sum(input.n);
        println!("{:?}", result);
    }
}

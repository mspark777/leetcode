struct Solution;

impl Solution {
    pub fn sum_of_the_digits_of_harshad_number(x: i32) -> i32 {
        let mut n = x;
        let mut sum = 0;
        while n > 0 {
            sum += n % 10;
            n /= 10;
        }

        match x % sum {
            0 => sum,
            _ => -1,
        }
    }
}

struct Input {
    x: i32,
}

fn main() {
    let inputs = [Input { x: 18 }, Input { x: 23 }];

    for input in inputs {
        let result = Solution::sum_of_the_digits_of_harshad_number(input.x);
        println!("{:?}", result);
    }
}

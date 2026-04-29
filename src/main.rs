struct Solution;

impl Solution {
    pub fn count_numbers_with_unique_digits(n: i32) -> i32 {
        if n == 0 {
            return 1;
        }

        let mut result = 10;
        let mut unique_digits = 9;
        let mut available_number = 9;

        for _ in 1..n {
            if available_number <= 0 {
                break;
            }
            unique_digits *= available_number;
            result += unique_digits;
            available_number -= 1;
        }

        result
    }
}

struct Input {
    n: i32,
}

fn main() {
    let inputs = [Input { n: 2 }, Input { n: 0 }];

    for input in inputs.into_iter() {
        let result = Solution::count_numbers_with_unique_digits(input.n);
        println!("{:?}", result);
    }
}

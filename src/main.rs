struct Solution;

impl Solution {
    pub fn clumsy(n: i32) -> i32 {
        match n {
            1 => 1,
            2 => 2 * 1,
            3 => 3 * 2 / 1,
            4 => 4 * 3 / 2 + 1,
            n => match n % 4 {
                0 => n + 1,
                1 | 2 => n + 2,
                3 => n - 1,
                _ => unreachable!(),
            },
        }
    }
}

struct Input {
    start_value: i32,
    target: i32,
}

fn main() {
    let inputs = [Input {
        start_value: 2,
        target: 3,
    }];

    for input in inputs {
        let result = Solution::broken_calc(input.start_value, input.target);
        println!("{:?}", result);
    }
}

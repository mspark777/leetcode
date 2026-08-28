struct Solution;

impl Solution {
    pub fn broken_calc(start_value: i32, target: i32) -> i32 {
        let m = 32 - ((target - 1) / start_value).leading_zeros() as i32;
        let diff = (start_value << m) - target;
        (diff >> m) + (diff & ((1 << m) - 1)).count_ones() as i32 + m
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

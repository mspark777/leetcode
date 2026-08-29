struct Solution;

impl Solution {
    pub fn is_valid(mut s: String) -> bool {
        while let Some(i) = s.find("abc") {
            s.replace_range(i..i + 3, "");
        }
        s.is_empty()
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

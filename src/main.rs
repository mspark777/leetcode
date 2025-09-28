struct Solution {}

impl Solution {
    pub fn is_same_after_reversals(num: i32) -> bool {
        (num == 0) || ((num % 10) != 0)
    }
}

struct Input {
    num: i32,
}

fn main() {
    let inputs = [Input { num: 526 }, Input { num: 1800 }, Input { num: 0 }];

    for input in inputs {
        let result = Solution::is_same_after_reversals(input.num);
        println!("{:?}", result);
    }
}

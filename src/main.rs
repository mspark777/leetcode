struct Solution;

impl Solution {
    pub fn count_commas(n: i32) -> i32 {
        0.max(n - 999)
    }
}

struct Input {
    n: i32,
}

fn main() {
    let inputs = [Input { n: 1002 }, Input { n: 998 }];

    for input in inputs {
        let result = Solution::count_commas(input.n);
        println!("{:?}", result);
    }
}

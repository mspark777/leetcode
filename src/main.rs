struct Solution;

impl Solution {
    pub fn count_monobit(n: i32) -> i32 {
        let n = (n as f64) + 1.0;
        (n.log2() as i32) + 1
    }
}

struct Input {
    n: i32,
}

fn main() {
    let inputs = [Input { n: 1 }, Input { n: 4 }];

    for input in inputs {
        let result = Solution::count_monobit(input.n);
        println!("{:?}", result);
    }
}

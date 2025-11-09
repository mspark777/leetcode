struct Solution {}

impl Solution {
    pub fn distinct_integers(n: i32) -> i32 {
        1.max(n - 1)
    }
}

struct Input {
    n: i32,
}

fn main() {
    let inputs = [Input { n: 5 }, Input { n: 3 }, Input { n: 886996 }];

    for input in inputs {
        let result = Solution::distinct_integers(input.n);
        println!("{:?}", result);
    }
}

struct Solution {}

impl Solution {
    pub fn smallest_even_multiple(n: i32) -> i32 {
        match n & 1 {
            1 => n * 2,
            _ => n,
        }
    }
}

struct Input {
    n: i32,
}

fn main() {
    let inputs = [Input { n: 5 }, Input { n: 6 }];

    for input in inputs {
        let result = Solution::smallest_even_multiple(input.n);
        println!("{:?}", result);
    }
}

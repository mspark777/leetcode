struct Solution;

impl Solution {
    pub fn smallest_number(n: i32) -> i32 {
        let mut result = 1;
        while result < n {
            result = (result << 1) | 1;
        }
        result
    }
}

struct Input {
    n: i32,
}

fn main() {
    let inputs = [Input { n: 5 }, Input { n: 10 }, Input { n: 3 }];

    for input in inputs {
        let result = Solution::smallest_number(input.n);
        println!("{:?}", result);
    }
}

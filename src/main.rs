struct Solution {}

impl Solution {
    pub fn number_of_cuts(n: i32) -> i32 {
        match n & 1 {
            0 => n / 2,
            _ => match n {
                1 => 0,
                _ => n,
            },
        }
    }
}

struct Input {
    n: i32,
}

fn main() {
    let inputs = [Input { n: 4 }, Input { n: 3 }];

    for input in inputs {
        let result = Solution::number_of_cuts(input.n);
        println!("{:?}", result);
    }
}

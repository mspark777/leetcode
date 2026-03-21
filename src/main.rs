struct Solution;

impl Solution {
    pub fn gcd_of_odd_even_sums(n: i32) -> i32 {
        n
    }
}

struct Input {
    n: i32,
}

fn main() {
    let inputs = [Input { n: 4 }, Input { n: 5 }];

    for input in inputs {
        let result = Solution::gcd_of_odd_even_sums(input.n);
        println!("{:?}", result);
    }
}

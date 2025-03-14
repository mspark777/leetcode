struct Solution {}

impl Solution {
    pub fn divisor_game(n: i32) -> bool {
        return n & 1 == 0;
    }
}

struct Input {
    n: i32,
}

fn main() {
    let inputs = vec![Input { n: 2 }, Input { n: 3 }];

    for input in inputs {
        let result = Solution::divisor_game(input.n);
        println!("{result:?}");
    }
}

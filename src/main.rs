struct Solution;

impl Solution {
    pub fn kth_grammar(_n: i32, k: i32) -> i32 {
        match (k - 1).count_ones() & 1 {
            0 => 0,
            _ => 1,
        }
    }
}

struct Input {
    n: i32,
    k: i32,
}

fn main() {
    let inputs = [Input { n: 1, k: 1 }];

    for input in inputs.into_iter() {
        let result = Solution::kth_grammar(input.n, input.k);
        println!("{:?}", result);
    }
}

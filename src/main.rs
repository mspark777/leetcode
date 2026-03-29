struct Solution;

impl Solution {
    pub fn minimum_flips(n: i32) -> i32 {
        let n = n as u32;
        let rev = n.reverse_bits() >> n.leading_zeros();
        (n ^ rev).count_ones() as i32
    }
}

struct Input {
    n: i32,
}

fn main() {
    let inputs = [Input { n: 7 }, Input { n: 10 }];

    for input in inputs {
        let result = Solution::minimum_flips(input.n);
        println!("{:?}", result);
    }
}

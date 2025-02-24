struct Solution {}

impl Solution {
    pub fn bitwise_complement(n: i32) -> i32 {
        let mut mask = 1;

        while mask < n {
            mask = (mask << 1) | 1;
        }

        return mask ^ n;
    }
}

struct Input {
    n: i32,
}

fn main() {
    let inputs = vec![Input { n: 5 }, Input { n: 7 }, Input { n: 10 }];

    for input in inputs {
        let result = Solution::bitwise_complement(input.n);
        println!("{result:?}");
    }
}

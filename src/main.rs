struct Solution {}

impl Solution {
    pub fn binary_gap(n: i32) -> i32 {
        let mut last = 33usize;
        let mut result = 0usize;

        for i in 0..32 {
            if (n >> i) & 1 == 1 {
                if last < 33 {
                    result = result.max(i - last);
                }

                last = i;
            }
        }

        return result as i32;
    }
}

struct Input {
    n: i32,
}

fn main() {
    let inputs = vec![Input { n: 22 }, Input { n: 8 }, Input { n: 5 }];

    for input in inputs {
        let result = Solution::binary_gap(input.n);
        println!("{result:?}");
    }
}

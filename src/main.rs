struct Solution {}

impl Solution {
    pub fn lexical_order(n: i32) -> Vec<i32> {
        let mut result = vec![0i32; n as usize];
        let mut cur = 1i32;
        let mut i = 0usize;

        while i < (n as usize) {
            result[i] = cur;

            if (cur * 10) <= n {
                cur *= 10;
                i += 1;
                continue;
            }

            while ((cur % 10) == 9) || (cur >= n) {
                cur /= 10;
            }

            cur += 1;
            i += 1;
        }

        return result;
    }
}

struct Input {
    n: i32,
}

fn main() {
    let inputs = vec![Input { n: 13 }, Input { n: 2 }];

    for input in inputs {
        let result = Solution::lexical_order(input.n);
        println!("{:?}", result);
    }
}

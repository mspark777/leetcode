struct Solution;

impl Solution {
    pub fn smallest_repunit_div_by_k(k: i32) -> i32 {
        if ((k & 1) == 0) || (k % 5 == 0) {
            return -1;
        }

        if k == 1 {
            return 1;
        }

        let mut r = 1;
        let mut len0 = 2;
        loop {
            r = ((10 * r) + 1) % k;
            if r == 0 {
                return len0;
            }
            len0 += 1;
        }
    }
}

struct Input {
    k: i32,
}

fn main() {
    let inputs = [Input { k: 1 }];

    for input in inputs {
        let result = Solution::smallest_repunit_div_by_k(input.k);
        println!("{:?}", result);
    }
}

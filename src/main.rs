struct Solution;

impl Solution {
    pub fn base_neg2(n: i32) -> String {
        let mut n2 = n as i64;
        let mut test_number = 2i64;
        while test_number <= n2 {
            if (n2 & test_number) != 0 {
                n2 += test_number << 1;
            }
            test_number <<= 2;
        }
        format!("{:b}", n2)
    }
}

struct Input {
    n: i32,
}

fn main() {
    let inputs = [Input { n: 3 }];

    for input in inputs {
        let result = Solution::base_neg2(input.n);
        println!("{:?}", result);
    }
}

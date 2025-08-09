struct Solution {}

impl Solution {
    pub fn get_no_zero_integers(n: i32) -> Vec<i32> {
        for i in 1..n {
            if Self::no_zero(i) && Self::no_zero(n - i) {
                return vec![i, n - i];
            }
        }

        vec![]
    }

    fn no_zero(n: i32) -> bool {
        let mut n = n;
        while n > 0 {
            if (n % 10) == 0 {
                return false;
            } else {
                n /= 10;
            }
        }

        true
    }
}

struct Input {
    n: i32,
}

fn main() {
    let inputs = [Input { n: 2 }, Input { n: 11 }];

    for input in inputs {
        let result = Solution::get_no_zero_integers(input.n);
        println!("{:?}", result);
    }
}

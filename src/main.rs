struct Solution {}

impl Solution {
    pub fn is_fascinating(n: i32) -> bool {
        let nums = [n, n * 2, n * 3];
        let mut visits = [false; 10];
        visits[0] = true;

        for mut digits in nums {
            while digits > 0 {
                let digit = (digits % 10) as usize;
                digits /= 10;

                if visits[digit] {
                    return false;
                } else {
                    visits[digit] = true;
                }
            }
        }

        true
    }
}

struct Input {
    n: i32,
}

fn main() {
    let inputs = [Input { n: 192 }, Input { n: 100 }];

    for input in inputs {
        let result = Solution::is_fascinating(input.n);
        println!("{:?}", result);
    }
}

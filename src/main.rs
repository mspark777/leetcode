struct Solution;

impl Solution {
    pub fn smallest_number(mut n: i32, t: i32) -> i32 {
        loop {
            let mut x = n;
            let mut product = 1;
            while x > 0 {
                product *= x % 10;
                x /= 10;
            }

            if (product % t) == 0 {
                break;
            }

            n += 1;
        }

        n
    }
}

struct Input {
    n: i32,
    t: i32,
}

fn main() {
    let inputs = [Input { n: 10, t: 2 }, Input { n: 15, t: 3 }];

    for input in inputs {
        let result = Solution::smallest_number(input.n, input.t);
        println!("{:?}", result);
    }
}

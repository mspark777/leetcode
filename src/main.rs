struct Solution;

impl Solution {
    pub fn judge_square_sum(c: i32) -> bool {
        let mut c = c;
        let mut i = 2;

        while (i * i) <= c {
            let mut count = 0;
            while (c % i) == 0 {
                count += 1;
                c /= i;
            }

            if ((i % 4) == 3) && ((count & 1) == 1) {
                return false;
            }

            i += 1;
        }

        (c % 4) != 3
    }
}

struct Input {
    c: i32,
}

fn main() {
    let inputs = [Input { c: 5 }, Input { c: 3 }];

    for input in inputs.into_iter() {
        let result = Solution::judge_square_sum(input.c);
        println!("{:?}", result);
    }
}

struct Solution {}

impl Solution {
    pub fn divisor_substrings(num: i32, k: i32) -> i32 {
        let mut result = 0;
        let mut cur = 0;
        let mut pow = 1;
        let mut n = num;
        let mut k = k;
        while n > 0 {
            cur += (n % 10) * pow;
            k -= 1;
            if k > 0 {
                pow *= 10;
            } else {
                if cur > 0 && num % cur == 0 {
                    result += 1
                }
                cur /= 10;
            }

            n /= 10;
        }

        result
    }
}

struct Input {
    num: i32,
    k: i32,
}

fn main() {
    let inputs = [Input { num: 240, k: 2 }, Input { num: 430043, k: 2 }];

    for input in inputs {
        let result = Solution::divisor_substrings(input.num, input.k);
        println!("{:?}", result);
    }
}

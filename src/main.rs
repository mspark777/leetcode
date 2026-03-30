struct Solution;

impl Solution {
    pub fn mirror_distance(n: i32) -> i32 {
        (n - Self::reverse(n)).abs()
    }

    fn reverse(mut n: i32) -> i32 {
        let mut r = 0;

        while n > 0 {
            let d = n % 10;
            n /= 10;

            r = r * 10 + d;
        }

        r
    }
}

struct Input {
    n: i32,
}

fn main() {
    let inputs = [Input { n: 25 }, Input { n: 10 }, Input { n: 7 }];

    for input in inputs {
        let result = Solution::mirror_distance(input.n);
        println!("{:?}", result);
    }
}

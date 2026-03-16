struct Solution;

impl Solution {
    pub fn min_cutting_cost(n: i32, m: i32, k: i32) -> i64 {
        let n = n as i64;
        let m = m as i64;
        let k = k as i64;
        let mut result = 0i64;

        if n > k {
            result += (n - k) * k;
        }

        if m > k {
            result += (m - k) * k;
        }

        result
    }
}

struct Input {
    n: i32,
    m: i32,
    k: i32,
}

fn main() {
    let inputs = [Input { n: 6, m: 5, k: 5 }, Input { n: 4, m: 4, k: 6 }];

    for input in inputs {
        let result = Solution::min_cutting_cost(input.n, input.m, input.k);
        println!("{:?}", result);
    }
}

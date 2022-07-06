pub struct Solution {}

impl Solution {
    pub fn fib(n: i32) -> i32 {
        if n < 2 {
            return n;
        }

        let mut prev0 = 0;
        let mut prev1 = 1;
        let mut cur = 1;
        for _ in 2..=n {
            cur = prev1 + prev0;
            prev0 = prev1;
            prev1 = cur;
        }

        cur
    }
}

struct Solution {}

impl Solution {
    pub fn num_tilings(n: i32) -> i32 {
        if n < 3 {
            return n;
        }

        let n = n as i64;
        const MOD: i64 = 1_000_000_007;
        let mut ways_prev1 = 2i64;
        let mut ways_prev2 = 1i64;
        let mut tromino_sum = 1i64;

        for _ in 3..=n {
            let current = (ways_prev1 + ways_prev2 + 2 * tromino_sum) % MOD;
            tromino_sum = (tromino_sum + ways_prev2) % MOD;
            ways_prev2 = ways_prev1;
            ways_prev1 = current;
        }

        return ways_prev1 as i32;
    }
}

struct Input {
    n: i32,
}

fn main() {
    let inputs = vec![Input { n: 3 }, Input { n: 1 }];

    for input in inputs {
        let result = Solution::num_tilings(input.n);
        println!("{result:?}");
    }
}

struct Solution {}
impl Solution {
    pub fn concatenated_binary(n: i32) -> i32 {
        let n = n as i64;
        let mut result = 1i64;
        let mut len = 4i64;
        const MOD: i64 = 1000000007;

        for i in 2..=n {
            if i == len {
                len *= 2;
            }

            result = ((result * len) + i) % MOD;
        }

        result as i32
    }
}

struct Input {
    n: i32,
}

fn main() {
    let inputs: Vec<Input> = vec![Input { n: 1 }, Input { n: 3 }, Input { n: 12 }];

    for Input { n } in inputs.into_iter() {
        let result = Solution::concatenated_binary(n);
        println!("{result:?}");
    }
}

struct Solution {}
impl Solution {
    pub fn unique_paths(m: i32, n: i32) -> i32 {
        let mut total = (m + n - 2) as u64;
        let r = (m.min(n) - 1) as u64;

        let mut steps = 1u64;

        for i in 1..=r {
            steps = steps * total / i;
            total -= 1;
        }

        steps as i32
    }
}

struct Input {
    m: i32,
    n: i32,
}

fn main() {
    let inputs: Vec<Input> = vec![
        Input { m: 3, n: 7 },
        Input { m: 3, n: 2 },
        Input { m: 51, n: 9 },
    ];

    for input in inputs {
        let m = input.m;
        let n = input.n;
        let result = Solution::unique_paths(m, n);
        println!("{:?}", result);
    }
}

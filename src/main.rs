struct Solution;

impl Solution {
    const MOD: i32 = 1337;
    pub fn super_pow(a: i32, b: Vec<i32>) -> i32 {
        if (a == 1) || (b[0] == 0) {
            return 1;
        }

        let a = a % Self::MOD;
        let mut xs = [0; 10];
        xs[0] = 1;
        for i in 1..10 {
            xs[i] = (xs[i - 1] * a) % Self::MOD;
        }
        let mut x0 = 1;
        for x in b {
            x0 = x0 * x0 % Self::MOD;
            let x1 = x0 * x0 % Self::MOD;
            x0 = x0 * x1 % Self::MOD;
            x0 = x0 * x1 % Self::MOD;
            x0 = x0 * xs[x as usize] % Self::MOD;
        }
        x0
    }
}

struct Input {
    a: i32,
    b: Vec<i32>,
}

fn main() {
    let inputs = [
        Input {
            a: 2,
            b: [3].to_vec(),
        },
        Input {
            a: 2,
            b: [1, 0].to_vec(),
        },
    ];

    for input in inputs.into_iter() {
        let result = Solution::super_pow(input.a, input.b);
        println!("{:?}", result);
    }
}

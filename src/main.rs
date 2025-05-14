struct Solution {}

const MOD: i64 = 1_000_000_007;
const L: usize = 26;

#[derive(Clone)]
struct Mat {
    a: [[i64; L]; L],
}

impl Mat {
    fn new() -> Self {
        return Self { a: [[0; L]; L] };
    }

    fn identity() -> Self {
        let mut m = Self::new();
        for i in 0..L {
            m.a[i][i] = 1;
        }
        return m;
    }
}

fn mul(left: &Mat, right: &Mat) -> Mat {
    let mut result = Mat::new();
    for i in 0..L {
        for j in 0..L {
            for k in 0..L {
                result.a[i][j] = (result.a[i][j] + left.a[i][k] * right.a[k][j]) % MOD;
            }
        }
    }
    return result;
}

fn quickmul(x: &Mat, mut y: i32) -> Mat {
    let mut ans = Mat::identity();
    let mut cur = x.clone();
    while y > 0 {
        if y & 1 == 1 {
            ans = mul(&ans, &cur);
        }
        cur = mul(&cur, &cur);
        y >>= 1;
    }
    return ans;
}

impl Solution {
    pub fn length_after_transformations(s: String, t: i32, nums: Vec<i32>) -> i32 {
        let mut mat = Mat::new();
        for i in 0..L {
            for j in 1..=nums[i] {
                let j = j as usize;
                mat.a[(i + j) % L][i] = 1;
            }
        }

        let res = quickmul(&mat, t);
        let mut f = [0; L];
        for ch in s.chars() {
            f[(ch as u8 - b'a') as usize] += 1;
        }
        let mut ans: i64 = 0;
        for i in 0..L {
            for j in 0..L {
                ans = (ans + res.a[i][j] * f[j]) % MOD;
            }
        }

        return ans as i32;
    }
}

struct Input {
    s: &'static str,
    t: i32,
    nums: Vec<i32>,
}

fn main() {
    let inputs = vec![
        Input {
            s: "abcyy",
            t: 2,
            nums: vec![
                1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 2,
            ],
        },
        Input {
            s: "azbk",
            t: 1,
            nums: vec![
                2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2,
            ],
        },
    ];

    for input in inputs {
        let result =
            Solution::length_after_transformations(input.s.to_string(), input.t, input.nums);
        println!("{result:?}");
    }
}

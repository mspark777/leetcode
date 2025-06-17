struct Solution {}

impl Solution {
    const MOD: i64 = 1000000007;
    const MX: usize = 100000;
    pub fn count_good_arrays(n: i32, m: i32, k: i32) -> i32 {
        let n = n as i64;
        let m = m as i64;
        let k = k as i64;
        let mut fact = vec![0i64; Self::MX];
        let mut inv_fact = vec![0i64; Self::MX];
        Self::init(&mut fact, &mut inv_fact);
        let result = Self::comb(&fact, &inv_fact, n - 1, k) * m % Self::MOD
            * Self::qpow(m - 1, n - k - 1)
            % Self::MOD;

        result as i32
    }

    fn init(fact: &mut Vec<i64>, inv_fact: &mut Vec<i64>) {
        fact[0] = 1;
        for i in 1..Self::MX {
            fact[i] = fact[i - 1] * (i as i64) % Self::MOD;
        }

        inv_fact[Self::MX - 1] = Self::qpow(fact[Self::MX - 1], Self::MOD - 2);
        for i in (1..Self::MX).rev() {
            inv_fact[i - 1] = inv_fact[i] * (i as i64) % Self::MOD;
        }
    }

    fn qpow(x: i64, n: i64) -> i64 {
        let mut x = x;
        let mut n = n;
        let mut res = 1i64;
        while n != 0 {
            if (n & 1) == 1 {
                res = res * x % Self::MOD;
            }

            x = x * x % Self::MOD;
            n >>= 1;
        }

        return res;
    }

    fn comb(fact: &Vec<i64>, inv_fact: &Vec<i64>, n: i64, m: i64) -> i64 {
        return fact[n as usize] * inv_fact[m as usize] % Self::MOD * inv_fact[(n - m) as usize]
            % Self::MOD;
    }
}

struct Input {
    n: i32,
    m: i32,
    k: i32,
}

fn main() {
    let inputs = vec![
        Input { n: 3, m: 2, k: 1 },
        Input { n: 4, m: 2, k: 2 },
        Input { n: 5, m: 2, k: 0 },
    ];

    for input in inputs {
        let result = Solution::count_good_arrays(input.n, input.m, input.k);
        println!("{:?}", result);
    }
}

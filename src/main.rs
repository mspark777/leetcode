struct Solution;

impl Solution {
    pub fn nth_super_ugly_number(n: i32, primes: Vec<i32>) -> i32 {
        if n == 1 {
            return 1;
        }

        let n = n as usize;
        let prime_count = primes.len();
        let mut prime_indices = vec![0usize; prime_count];
        let mut uglys = vec![0i64; n];
        uglys[0] = 1;

        for i in 1..n {
            let mut min_val = i64::MAX;

            for j in 0..prime_count {
                let prime = primes[j] as i64;
                let ugly = uglys[prime_indices[j]];
                min_val = min_val.min(prime * ugly);
            }

            uglys[i] = min_val;

            for j in 0..prime_count {
                let prime = primes[j] as i64;
                let ugly = uglys[prime_indices[j]];
                let val = prime * ugly;
                if min_val == val {
                    prime_indices[j] += 1;
                }
            }
        }

        uglys[n - 1] as i32
    }
}

struct Input {
    n: i32,
    primes: Vec<i32>,
}

fn main() {
    let inputs = [Input {
        n: 12,
        primes: [2, 7, 13, 19].to_vec(),
    }];

    for input in inputs {
        let result = Solution::nth_super_ugly_number(input.n, input.primes);
        println!("{:?}", result);
    }
}

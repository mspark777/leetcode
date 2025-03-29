struct Solution {}

const MOD: i64 = 1000000007;

impl Solution {
    pub fn maximum_score(nums: Vec<i32>, k: i32) -> i32 {
        let n = nums.len();
        let mut prime_scores = vec![0usize; n];

        let max_element = nums.iter().cloned().max().unwrap();
        let primes = Self::get_primes(max_element as usize);

        for (i, num) in nums.iter().cloned().enumerate() {
            let mut num = num as usize;
            for prime in primes.iter().cloned() {
                if (prime * prime) > num {
                    break;
                } else if (num % prime) != 0 {
                    continue;
                }

                prime_scores[i] += 1;
                while (num % prime) == 0 {
                    num /= prime;
                }
            }

            if num > 1 {
                prime_scores[i] += 1;
            }
        }

        let mut next_dominant = vec![n as i64; n];
        let mut prev_dominant = vec![-1i64; n];
        let mut decreasing_prime_score_stack = Vec::<i32>::new();

        for i in 0..n {
            while let Some(&top) = decreasing_prime_score_stack.last() {
                if prime_scores[top as usize] >= prime_scores[i] {
                    break;
                }

                let top_idx = decreasing_prime_score_stack.pop().unwrap();
                next_dominant[top_idx as usize] = i as i64;
            }

            if let Some(&top) = decreasing_prime_score_stack.last() {
                prev_dominant[i] = top as i64;
            }

            decreasing_prime_score_stack.push(i as i32);
        }

        let mut num_of_subarrays = vec![0i64; n];
        for i in 0..n {
            let j = i as i64;
            num_of_subarrays[i] = (next_dominant[i] - j) * (j - prev_dominant[i]);
        }

        let mut sorted_array = nums
            .iter()
            .cloned()
            .enumerate()
            .map(|(i, n)| (n, i))
            .collect::<Vec<_>>();

        sorted_array.sort_unstable_by_key(|k| -k.0);

        let mut result = 1i64;
        let mut processing_index = 0usize;

        let mut k = k as i64;
        while k > 0 {
            let (num, i) = sorted_array[processing_index];
            processing_index += 1;
            let operations = k.min(num_of_subarrays[i]);

            result = (result * Self::power(num as i64, operations)) % MOD;
            k -= operations;
        }

        return result as i32;
    }

    fn power(base: i64, exponent: i64) -> i64 {
        let mut base = base;
        let mut exponent = exponent;
        let mut res = 1i64;

        while exponent > 0 {
            if (exponent & 1) == 1 {
                res = (res * base) % MOD;
            }

            base = (base * base) % MOD;
            exponent /= 2;
        }

        return res;
    }

    fn get_primes(limit: usize) -> Vec<usize> {
        let mut is_prime = vec![true; limit + 1];
        let mut primes = Vec::<usize>::with_capacity(limit);

        for num in 2..limit {
            if !is_prime[num] {
                continue;
            }

            primes.push(num);

            for multiple in ((num * num)..=limit).step_by(num) {
                is_prime[multiple] = false;
            }
        }

        return primes;
    }
}

struct Input {
    nums: Vec<i32>,
    k: i32,
}

fn main() {
    let inputs = vec![
        Input {
            nums: vec![8, 3, 9, 3, 8],
            k: 2,
        },
        Input {
            nums: vec![19, 12, 14, 6, 10, 18],
            k: 3,
        },
    ];

    for input in inputs {
        let result = Solution::maximum_score(input.nums, input.k);
        println!("{result:?}");
    }
}

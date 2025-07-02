struct Solution {}

impl Solution {
    pub fn possible_string_count(word: String, k: i32) -> i32 {
        let k = k as usize;
        const MOD: i64 = 1_000_000_007;
        let word = word.chars().collect::<Vec<char>>();
        let n = word.len();
        let mut count = 1;
        let mut frequencies = Vec::<i32>::new();
        for i in 1..n {
            if word[i] == word[i - 1] {
                count += 1;
            } else {
                frequencies.push(count);
                count = 1;
            }
        }
        frequencies.push(count);

        let mut result: i64 = 1;
        for frequency in frequencies.iter().cloned() {
            let f = frequency as i64;
            result = (result * f) % MOD;
        }

        if frequencies.len() >= k {
            return result as i32;
        }

        let mut g = vec![1; k];
        for num in frequencies.iter().cloned() {
            let mut f_new = vec![0; k];
            for j in 1..k {
                f_new[j] = g[j - 1];
                if j as i32 - num - 1 >= 0 {
                    f_new[j] = (f_new[j] - g[j - num as usize - 1] + MOD) % MOD;
                }
            }

            let mut g_new = vec![0; k];
            g_new[0] = f_new[0];
            for j in 1..k {
                g_new[j] = (g_new[j - 1] + f_new[j]) % MOD;
            }

            g = g_new;
        }

        ((result - g[k - 1] as i64 + MOD) % MOD) as i32
    }
}

struct Input {
    word: &'static str,
    k: i32,
}

fn main() {
    let inputs = vec![
        Input {
            word: "aabbccdd",
            k: 7,
        },
        Input {
            word: "aabbccdd",
            k: 8,
        },
        Input {
            word: "aaabbb",
            k: 3,
        },
    ];

    for input in inputs {
        let result = Solution::possible_string_count(input.word.to_string(), input.k);
        println!("{:?}", result);
    }
}

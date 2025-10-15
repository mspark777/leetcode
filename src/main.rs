struct Solution {}

impl Solution {
    pub fn remove_anagrams(words: Vec<String>) -> Vec<String> {
        let mut result = Vec::<String>::new();
        result.push(words[0].clone());
        let mut prev = Self::prime_hash(words[0].as_str());

        for word in words.iter().skip(1) {
            let hash = Self::prime_hash(word.as_str());
            if hash != prev {
                result.push(word.clone());
                prev = hash;
            }
        }

        result
    }

    fn prime_hash(s: &str) -> i32 {
        let primes = [
            1, 2, 3, 5, 7, 11, 13, 17, 19, 23, 29, 31, 37, 41, 43, 47, 53, 59, 61, 67, 71, 73, 79,
            83, 89, 97,
        ];

        const MOD: i64 = 1000000007;
        let mut hash = 1i64;
        for c in s.chars() {
            let code = c as usize;
            let idx = code - 'a' as usize;
            hash = (hash * primes[idx]) % MOD;
        }
        hash as i32
    }
}

struct Input {
    words: Vec<String>,
}

fn main() {
    let inputs = [
        Input {
            words: ["abba", "baba", "bbaa", "cd", "cd"]
                .map(|s| s.to_string())
                .to_vec(),
        },
        Input {
            words: ["a", "b", "c", "d", "e"].map(|s| s.to_string()).to_vec(),
        },
    ];

    for input in inputs {
        let result = Solution::remove_anagrams(input.words);
        println!("{:?}", result);
    }
}

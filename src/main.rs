struct Solution;

impl Solution {
    pub fn max_freq_sum(s: String) -> i32 {
        const A: usize = 'a' as usize;
        let mut frequencies = [0i32; 26];

        for ch in s.chars() {
            let code = ch as usize;
            let idx = code - A;
            frequencies[idx] += 1;
        }

        let mut max_vowel = 0;
        for vowel in [b'a', b'e', b'i', b'o', b'u'] {
            let idx = (vowel as usize) - A;
            max_vowel = max_vowel.max(frequencies[idx]);
            frequencies[idx] = 0;
        }

        max_vowel + frequencies.iter().copied().max().unwrap_or_default()
    }
}

struct Input {
    s: String,
}

fn main() {
    let inputs = [
        Input {
            s: "successes".to_string(),
        },
        Input {
            s: "aeiaeia".to_string(),
        },
    ];

    for input in inputs {
        let result = Solution::max_freq_sum(input.s);
        println!("{:?}", result);
    }
}

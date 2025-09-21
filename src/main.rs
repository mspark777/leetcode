struct Solution {}

impl Solution {
    pub fn count_vowel_substrings(word: String) -> i32 {
        let s = word.chars().collect::<Vec<char>>();
        return Self::at_most_k(&s, 5) - Self::at_most_k(&s, 4);
    }

    fn at_most_k(s: &Vec<char>, k: i32) -> i32 {
        let k = k as usize;
        let mut result = 0;
        let mut i = 0usize;
        let mut counts = std::collections::HashMap::<char, i32>::new();

        for (j, ch) in s.iter().cloned().enumerate() {
            if !Self::is_vowel(ch) {
                i = j + 1;
                counts.clear();
                continue;
            }

            counts.entry(ch).and_modify(|c| *c += 1).or_insert(1);
            while counts.len() > k {
                let left = s[i];
                counts.entry(left).and_modify(|c| *c -= 1);
                if let Some(&c) = counts.get(&left) {
                    if c == 0 {
                        counts.remove(&left);
                    }
                }
                i += 1;
            }

            result += j + 1 - i;
        }

        result as i32
    }

    fn is_vowel(c: char) -> bool {
        c == 'a' || c == 'e' || c == 'i' || c == 'o' || c == 'u'
    }
}

struct Input {
    word: String,
}

fn main() {
    let inputs = [
        Input {
            word: "aeiouu".to_string(),
        },
        Input {
            word: "unicornarihan".to_string(),
        },
        Input {
            word: "cuaieuouac".to_string(),
        },
    ];

    for input in inputs {
        let result = Solution::count_vowel_substrings(input.word);
        println!("{:?}", result);
    }
}

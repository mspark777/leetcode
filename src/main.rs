struct Solution {}

impl Solution {
    pub fn count_of_substrings(word: String, k: i32) -> i64 {
        let word = word.chars().collect::<Vec<char>>();
        let k = k as usize;
        return Self::at_least_k(&word, k) - Self::at_least_k(&word, k + 1);
    }

    fn is_vowel(c: char) -> bool {
        return c == 'a' || c == 'e' || c == 'i' || c == 'o' || c == 'u';
    }

    fn at_least_k(word: &Vec<char>, k: usize) -> i64 {
        let mut result = 0usize;
        let mut start = 0usize;
        let mut end = 0usize;
        let mut vowel_count = std::collections::HashMap::<char, usize>::new();
        let mut consonant_count = 0usize;

        while end < word.len() {
            let new_letter = word[end];
            if Self::is_vowel(new_letter) {
                vowel_count
                    .entry(new_letter)
                    .and_modify(|x| *x += 1)
                    .or_insert(1);
            } else {
                consonant_count += 1;
            }

            while vowel_count.len() == 5 && consonant_count >= k {
                result += word.len() - end;
                let start_letter = word[start];
                if Self::is_vowel(start_letter) {
                    let count = vowel_count.get_mut(&start_letter).unwrap();
                    if *count <= 1 {
                        vowel_count.remove(&start_letter);
                    } else {
                        *count -= 1;
                    }
                } else {
                    consonant_count -= 1;
                }
                start += 1;
            }

            end += 1;
        }

        return result as i64;
    }
}

struct Input {
    word: &'static str,
    k: i32,
}

fn main() {
    let inputs = vec![
        Input {
            word: "aeioqq",
            k: 1,
        },
        Input {
            word: "aeiou",
            k: 0,
        },
        Input {
            word: "ieaouqqieaouqq",
            k: 1,
        },
    ];

    for input in inputs {
        let result = Solution::count_of_substrings(input.word.to_string(), input.k);
        println!("{result:?}");
    }
}

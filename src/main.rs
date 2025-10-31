struct Solution {}

impl Solution {
    pub fn equal_frequency(word: String) -> bool {
        let mut frequencies = vec![0; 26];
        for ch in word.chars() {
            let a = 'a' as usize;
            let code = ch as usize;
            let idx = code - a;
            frequencies[idx] += 1;
        }

        for (i, frequency) in frequencies.iter().cloned().enumerate() {
            if frequency == 0 {
                continue;
            }

            let mut set = std::collections::HashSet::<i32>::with_capacity(26);
            for (j, freq) in frequencies.iter().cloned().enumerate() {
                let mut f = freq;
                if i == j {
                    f -= 1;
                }

                if f > 0 {
                    set.insert(f);
                }
            }

            if set.len() == 1 {
                return true;
            }
        }

        return false;
    }
}

struct Input {
    word: String,
}

fn main() {
    let inputs = [
        Input {
            word: "abcc".to_string(),
        },
        Input {
            word: "aazz".to_string(),
        },
        Input {
            word: "bbac".to_string(),
        },
    ];

    for input in inputs {
        let result = Solution::equal_frequency(input.word);
        println!("{:?}", result);
    }
}

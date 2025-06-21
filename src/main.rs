struct Solution {}

impl Solution {
    pub fn minimum_deletions(word: String, k: i32) -> i32 {
        let mut counts = vec![0; 26];
        let mut result = 0;
        for ch in word.chars() {
            result += 1;
            let idx = ((ch as u8) - b'a') as usize;
            counts[idx] += 1;
        }

        for a in counts.iter().cloned() {
            if a < 1 {
                continue;
            }

            let mut deleted = 0;
            for b in counts.iter().cloned() {
                if b < 1 {
                    continue;
                }

                if a > b {
                    deleted += b;
                } else if b > (a + k) {
                    deleted += b - (a + k);
                }
            }

            result = result.min(deleted);
        }

        result
    }
}

struct Input {
    word: &'static str,
    k: i32,
}

fn main() {
    let inputs = vec![
        Input {
            word: "aabcaba",
            k: 0,
        },
        Input {
            word: "dabdcbdcdcd",
            k: 2,
        },
        Input {
            word: "aaabaaa",
            k: 2,
        },
    ];

    for input in inputs {
        let result = Solution::minimum_deletions(input.word.to_string(), input.k);
        println!("{:?}", result);
    }
}

struct Solution {}

impl Solution {
    pub fn similar_pairs(words: Vec<String>) -> i32 {
        let mut result = 0;
        let mut masks = std::collections::HashMap::<u32, i32>::new();

        for word in words.iter() {
            let mut mask = 0u32;
            for ch in word.chars() {
                let a = 'a' as u32;
                let code = ch as u32;
                let shift = code - a;
                mask |= 1 << shift;
            }

            let count = masks.entry(mask).or_insert(0);
            result += *count;
            *count += 1;
        }

        result
    }
}

struct Input {
    words: Vec<String>,
}

fn main() {
    let inputs = [
        Input {
            words: ["aba", "aabb", "abcd", "bac", "aabc"]
                .map(|r| r.to_string())
                .to_vec(),
        },
        Input {
            words: ["aabb", "ab", "ba"].map(|r| r.to_string()).to_vec(),
        },
        Input {
            words: ["nba", "cba", "dba"].map(|r| r.to_string()).to_vec(),
        },
    ];

    for input in inputs {
        let result = Solution::similar_pairs(input.words);
        println!("{:?}", result);
    }
}

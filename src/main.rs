struct Solution;

impl Solution {
    pub fn num_special_equiv_groups(words: Vec<String>) -> i32 {
        use std::collections::HashSet;

        let mut seen = HashSet::<String>::new();
        for word in words {
            let mut counts = [0; 52];
            for (i, ch) in word.char_indices() {
                let code = ch as usize;
                const A: usize = 'a' as usize;
                let idx = code - A + (26 * (i & 1));
                counts[idx] += 1;
            }

            seen.insert(format!("{:?}", counts));
        }

        seen.len() as i32
    }
}

struct Input {
    words: Vec<String>,
}

fn main() {
    let inputs = [Input {
        words: ["abcd", "cdab", "cbad", "xyzz", "zzxy", "zzyx"]
            .map(|v| v.to_string())
            .to_vec(),
    }];

    for input in inputs {
        let result = Solution::num_special_equiv_groups(input.words);
        println!("{:?}", result);
    }
}

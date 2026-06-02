struct Solution;

impl Solution {
    pub fn find_longest_word(s: String, mut d: Vec<String>) -> String {
        d.sort_unstable_by(|a, b| b.len().cmp(&a.len()).then_with(|| a.cmp(b)));
        for s2 in d {
            let mut s2_iter = s2.chars().peekable();
            for c in s.chars() {
                if let Some(&c2) = s2_iter.peek() {
                    if c == c2 {
                        s2_iter.next();
                    }
                } else {
                    break;
                }
            }
            if s2_iter.peek().is_none() {
                return s2;
            }
        }
        String::new()
    }
}

struct Input {}

fn main() {
    let inputs = [
        Input {
            strs: ["aba", "cdc", "eae"].map(|s| s.to_string()).to_vec(),
        },
        Input {
            strs: ["aaa", "aaa", "aa"].map(|s| s.to_string()).to_vec(),
        },
        Input {
            strs: ["aabbcc", "aabbcc", "c"].map(|s| s.to_string()).to_vec(),
        },
    ];

    for input in inputs.into_iter() {
        let result = Solution::find_lu_slength(input.strs);
        println!("{:?}", result);
    }
}

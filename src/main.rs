struct Solution;

impl Solution {
    pub fn is_acronym(words: Vec<String>, s: String) -> bool {
        let mut word = words.iter();
        let mut ch = s.chars();
        loop {
            match (word.next(), ch.next()) {
                (Some(w), Some(c)) if w.chars().nth(0).unwrap_or(' ') == c => continue,
                (None, None) => break,
                _ => return false,
            };
        }

        true
    }
}

struct Input {
    words: Vec<String>,
    s: String,
}

fn main() {
    let inputs = [
        Input {
            words: ["alice", "bob", "charlie"].map(|s| s.to_string()).to_vec(),
            s: "abc".to_string(),
        },
        Input {
            words: ["an", "apple"].map(|s| s.to_string()).to_vec(),
            s: "a".to_string(),
        },
        Input {
            words: ["never", "gonna", "give", "up", "on", "you"]
                .map(|s| s.to_string())
                .to_vec(),
            s: "ngguoy".to_string(),
        },
    ];

    for input in inputs {
        let result = Solution::is_acronym(input.words, input.s);
        println!("{:?}", result);
    }
}

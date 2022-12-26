use std::collections::HashMap;

struct Solution {}
impl Solution {
    pub fn word_pattern(pattern: String, s: String) -> bool {
        let patterns: Vec<String> = pattern.chars().map(|c| c.to_string()).collect();
        let words: Vec<String> = s.split(" ").map(|s| s.to_string()).collect();

        if patterns.len() != words.len() {
            return false;
        }

        let mut ptow = HashMap::<String, String>::new();
        let mut wtop = HashMap::<String, String>::new();

        for i in 0..words.len() {
            let word = &words[i];
            let p = &patterns[i];

            if let Some(w) = ptow.get(p) {
                if w != word {
                    return false;
                }
            } else {
                ptow.insert(p.clone(), word.clone());
            }

            if let Some(pp) = wtop.get(word) {
                if pp != p {
                    return false;
                }
            } else {
                wtop.insert(word.clone(), p.clone());
            }
        }

        return true;
    }
}

fn main() {
    let inputs = [
        ("abba", "dog cat cat dog"),
        ("abba", "dog cat cat fish"),
        ("aaaa", "dog cat cat dog"),
        ("abba", "dog dog dog dog"),
    ];

    for (p, s) in inputs {
        let result = Solution::word_pattern(p.to_string(), s.to_string());
        println!("{result}");
    }
}

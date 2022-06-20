use std::collections::HashSet;

struct Solution {}
impl Solution {
    pub fn minimum_length_encoding(words: Vec<String>) -> i32 {
        let mut filter = HashSet::<String>::new();

        for word in &words {
            filter.insert(word.clone());
        }

        for word in &words {
            for i in 1..word.len() {
                let sub = &word[i..];
                filter.remove(sub);
            }
        }

        let mut result = 0usize;
        for word in filter {
            result += word.len() + 1;
        }

        result as i32
    }
}

fn main() {
    let inputs = [vec!["time", "me", "bell"], vec!["t"]];

    for input in inputs {
        let words = input.iter().map(|s| String::from(*s)).collect();
        let result = Solution::minimum_length_encoding(words);
        println!("{result:?}");
    }
}

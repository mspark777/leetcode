use std::iter::FromIterator;

struct Solution {}
impl Solution {
    pub fn reverse_words(s: String) -> String {
        let mut result: Vec<String> = vec![];
        for word in s.split(" ") {
            let w: String = String::from_iter(word.chars().rev());
            result.push(w);
        }

        result.join(" ")
    }
}

struct Input {
    s: &'static str,
}

fn main() {
    let inputs: Vec<Input> = vec![
        Input {
            s: "Let's take LeetCode contest",
        },
        Input { s: "God Ding" },
    ];

    for Input { s } in inputs.into_iter() {
        let result = Solution::reverse_words(s.to_string());
        println!("{result:?}");
    }
}

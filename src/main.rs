struct Solution {}
impl Solution {
    pub fn detect_capital_use(word: String) -> bool {
        if word == word.to_uppercase() {
            return true;
        } else if word == word.to_lowercase() {
            return true;
        }

        let mut chars = word.chars();
        let first = chars.next().unwrap().to_string().to_uppercase();
        let remains: String = chars.into_iter().collect();

        return word == format!("{}{}", first, remains.to_lowercase());
    }
}

fn main() {
    let inputs = ["USA", "Google", "leetcode", "FlaG"];

    for word in inputs {
        let result = Solution::detect_capital_use(word.to_string());
        println!("{result}");
    }
}

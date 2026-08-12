struct Solution;

impl Solution {
    pub fn expressive_words(s: String, words: Vec<String>) -> i32 {
        let s = s.chars().collect::<Vec<char>>();
        let mut result = 0;
        for w in words {
            if Self::can_express(&s, w.chars().collect()) {
                result += 1;
            }
        }
        result
    }

    fn can_express(s: &[char], w: Vec<char>) -> bool {
        let mut i = 0;
        let mut j = 0;
        while i < s.len() && j < w.len() {
            let mut cnt_s = 1;
            let mut cnt_w = 1;
            if s[i] != w[j] {
                return false;
            }
            while i < s.len() - 1 && s[i] == s[i + 1] {
                cnt_s += 1;
                i += 1;
            }
            while j < w.len() - 1 && w[j] == w[j + 1] {
                cnt_w += 1;
                j += 1;
            }
            if cnt_s < cnt_w || ((cnt_s - cnt_w) > 0 && cnt_s < 3) {
                return false;
            }
            i += 1;
            j += 1;
        }
        i == s.len() && j == w.len()
    }
}

struct Input {
    s: String,
    words: Vec<String>,
}

fn main() {
    let inputs = [Input {
        s: "heeellooo".to_string(),
        words: ["hello", "hi", "helo"].map(|v| v.to_string()).to_vec(),
    }];

    for input in inputs {
        let result = Solution::expressive_words(input.s, input.words);
        println!("{:?}", result);
    }
}

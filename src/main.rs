struct Solution;

impl Solution {
    pub fn letter_case_permutation(s: String) -> Vec<String> {
        let mut chars = s.chars().collect::<Vec<char>>();
        let mut answer = Vec::new();
        Self::dfs(&mut chars, &mut answer, 0);
        answer
    }
    fn dfs(chars: &mut [char], answer: &mut Vec<String>, i: usize) {
        if i == chars.len() {
            answer.push(chars.iter().collect());
        } else {
            Self::dfs(chars, answer, i + 1);
            if chars[i].is_alphabetic() {
                chars[i] = ((chars[i] as u8) ^ (1 << 5)) as char;
                Self::dfs(chars, answer, i + 1);
                chars[i] = ((chars[i] as u8) ^ (1 << 5)) as char;
            }
        }
    }
}

struct Input {
    s: String,
}

fn main() {
    let inputs = [Input {
        s: "a1b2".to_string(),
    }];

    for input in inputs.into_iter() {
        let result = Solution::letter_case_permutation(input.s);
        println!("{:?}", result);
    }
}

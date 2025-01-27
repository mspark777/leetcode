struct Solution {}

impl Solution {
    pub fn reverse_only_letters(s: String) -> String {
        let s = s.chars().collect::<Vec<char>>();
        let mut chars = Vec::<char>::with_capacity(s.len());

        let mut j = (s.len() as i32) - 1;
        for i in 0..s.len() {
            let left = s[i];
            if Self::is_letter(left) {
                while j >= 0 && !Self::is_letter(s[j as usize]) {
                    j -= 1;
                }

                if j >= 0 {
                    chars.push(s[j as usize]);
                    j -= 1;
                }
            } else {
                chars.push(left);
            }
        }

        return chars.iter().collect();
    }

    fn is_letter(ch: char) -> bool {
        let ch = ch as u8;

        return ((b'a' <= ch) && (ch <= b'z')) || ((b'A' <= ch) && (ch <= b'Z'));
    }
}

struct Input {
    s: &'static str,
}

fn main() {
    let inputs = vec![
        Input { s: "ab-cd" },
        Input { s: "a-bC-dEf-ghIj" },
        Input {
            s: "Test1ng-Leet=code-Q!",
        },
    ];

    for input in inputs {
        let result = Solution::reverse_only_letters(input.s.to_string());
        println!("{result:?}");
    }
}

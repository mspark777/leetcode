struct Solution {}

impl Solution {
    pub fn make_fancy_string(s: String) -> String {
        let mut chars: Vec<char> = s.chars().collect();
        if chars.len() < 3 {
            return s;
        }

        let mut j = 2usize;

        for i in 2..chars.len() {
            if chars[i] != chars[j - 1] || chars[i] != chars[j - 2] {
                chars[j] = chars[i];
                j += 1;
            }
        }

        return chars.iter().take(j).collect();
    }
}

struct Input {
    s: String,
}

fn main() {
    let inputs = vec![
        Input {
            s: "leeetcode".to_string(),
        },
        Input {
            s: "aaabaaaa".to_string(),
        },
        Input {
            s: "aab".to_string(),
        },
    ];

    for input in inputs {
        let result = Solution::make_fancy_string(input.s);
        println!("{result}");
    }
}

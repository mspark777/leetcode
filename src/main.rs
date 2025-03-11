struct Solution {}

impl Solution {
    pub fn remove_outer_parentheses(s: String) -> String {
        let mut result = Vec::<char>::new();
        let mut balance = 0;

        for ch in s.chars() {
            if ch == '(' {
                if balance > 0 {
                    result.push(ch);
                }

                balance += 1;
            } else {
                balance -= 1;

                if balance > 0 {
                    result.push(ch);
                }
            }
        }

        return result.iter().collect();
    }
}

struct Input {
    s: &'static str,
}

fn main() {
    let inputs = vec![
        Input { s: "(()())(())" },
        Input {
            s: "(()())(())(()(()))",
        },
        Input { s: "()()" },
    ];

    for input in inputs {
        let result = Solution::remove_outer_parentheses(input.s.to_string());
        println!("{result:?}");
    }
}

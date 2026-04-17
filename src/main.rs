struct Solution;

impl Solution {
    pub fn calculate(s: String) -> i32 {
        let n = s.len();
        let mut current = 0;
        let mut last = 0;
        let mut result = 0;
        let mut sign = '+';
        for (i, ch) in s.chars().enumerate() {
            if ch.is_ascii_digit() {
                let code = ch as u8;
                current = current * 10 + ((code - b'0') as i32);
            }

            if (!ch.is_ascii_digit() && !ch.is_ascii_whitespace()) || ((i + 1) == n) {
                if (sign == '+') || (sign == '-') {
                    result += last;
                    last = match sign {
                        '+' => current,
                        _ => -current,
                    };
                } else if sign == '*' {
                    last *= current;
                } else if sign == '/' {
                    last /= current;
                }

                sign = ch;
                current = 0;
            }
        }

        result += last;
        result
    }
}

struct Input {
    s: String,
}

fn main() {
    let inputs = [
        Input {
            s: "3+2*2".to_string(),
        },
        Input {
            s: " 3+5 / 2 ".to_string(),
        },
    ];

    for input in inputs {
        let result = Solution::calculate(input.s);
        println!("{:?}", result);
    }
}

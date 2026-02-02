struct Solution;

impl Solution {
    pub fn find_latest_time(s: String) -> String {
        let mut digits = s.chars().collect::<Vec<char>>();

        if digits[0] == '?' {
            if (digits[1] != '0') && (digits[1] != '1') && (digits[1] != '?') {
                digits[0] = '0';
            } else {
                digits[0] = '1';
            }
        }

        if digits[1] == '?' {
            if digits[0] == '1' {
                digits[1] = '1';
            } else {
                digits[1] = '9';
            }
        }

        if digits[3] == '?' {
            digits[3] = '5';
        }

        if digits[4] == '?' {
            digits[4] = '9';
        }

        digits.iter().collect()
    }
}

struct Input {
    s: String,
}

fn main() {
    let inputs = [
        Input {
            s: "1?:?4".to_string(),
        },
        Input {
            s: "0?:5?".to_string(),
        },
    ];

    for input in inputs {
        let result = Solution::find_latest_time(input.s);
        println!("{:?}", result);
    }
}

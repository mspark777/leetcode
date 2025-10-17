struct Solution {}

impl Solution {
    pub fn strong_password_checker_ii(password: String) -> bool {
        let mut n = 0usize;
        let mut lowercase = false;
        let mut uppercase = false;
        let mut digit = false;
        let mut special = false;
        let mut prev: Option<char> = None;

        for c in password.chars() {
            if c.is_ascii_lowercase() {
                lowercase = true;
            } else if c.is_ascii_uppercase() {
                uppercase = true;
            } else if c.is_ascii_digit() {
                digit = true;
            } else {
                special = true;
            }

            if let Some(p) = prev {
                if c == p {
                    return false;
                }
            }

            prev = Some(c);
            n += 1;
        }

        return n >= 8 && lowercase && uppercase && digit && special;
    }
}

struct Input {
    password: String,
}

fn main() {
    let inputs = [
        Input {
            password: "IloveLe3tcode!".to_string(),
        },
        Input {
            password: "Me+You--IsMyDream".to_string(),
        },
        Input {
            password: "1aB!".to_string(),
        },
    ];

    for input in inputs {
        let result = Solution::strong_password_checker_ii(input.password);
        println!("{:?}", result);
    }
}

struct Solution {}

impl Solution {
    pub fn can_be_valid(s: String, locked: String) -> bool {
        let s = s.chars().collect::<Vec<char>>();
        let locked = locked.chars().collect::<Vec<char>>();

        if s.len() & 1 == 1 {
            return false;
        }

        let mut open_brackets = 0;
        let mut unlocked = 0;

        for (&bracket, &lock) in s.iter().zip(locked.iter()) {
            if lock == '0' {
                unlocked += 1;
            } else if bracket == '(' {
                open_brackets += 1;
            } else if bracket == ')' {
                if open_brackets > 0 {
                    open_brackets -= 1;
                } else if unlocked > 0 {
                    unlocked -= 1;
                } else {
                    return false;
                }
            }
        }

        let mut balance = 0;

        for (&bracket, &lock) in s.iter().rev().zip(locked.iter().rev()) {
            if lock == '0' {
                balance -= 1;
                unlocked -= 1;
            } else if bracket == '(' {
                balance += 1;
                open_brackets -= 1;
            } else if bracket == ')' {
                balance -= 1;
            }

            if balance > 0 {
                return false;
            }

            if unlocked == 0 && open_brackets == 0 {
                break;
            }
        }

        return if open_brackets <= 0 { true } else { false };
    }
}

struct Input {
    s: &'static str,
    locked: &'static str,
}

fn main() {
    let inputs = vec![
        Input {
            s: "))()))",
            locked: "010100",
        },
        Input {
            s: "()()",
            locked: "0000",
        },
        Input {
            s: ")",
            locked: "0",
        },
    ];

    for input in inputs {
        let result = Solution::can_be_valid(input.s.to_string(), input.locked.to_string());
        println!("{result:?}");
    }
}

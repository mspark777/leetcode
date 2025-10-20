struct Solution {}

impl Solution {
    pub fn count_asterisks(s: String) -> i32 {
        let mut skip = false;
        let mut result = 0;

        for c in s.chars() {
            if c == '|' {
                skip = !skip;
                continue;
            }

            if skip {
                continue;
            }

            if c == '*' {
                result += 1;
            }
        }

        result
    }
}

struct Input {
    s: String,
}

fn main() {
    let inputs = [
        Input {
            s: "l|*e*et|c**o|*de|".to_string(),
        },
        Input {
            s: "iamprogrammer".to_string(),
        },
        Input {
            s: "yo|uar|e**|b|e***au|tifu|l".to_string(),
        },
    ];

    for input in inputs {
        let result = Solution::count_asterisks(input.s);
        println!("{:?}", result);
    }
}

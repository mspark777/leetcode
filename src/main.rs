struct Solution {}

impl Solution {
    pub fn min_changes(s: String) -> i32 {
        let mut chars = s.chars();
        let mut result = 0;

        while let Some(first) = chars.next() {
            if let Some(second) = chars.next() {
                if first != second {
                    result += 1;
                }
            } else {
                break;
            }
        }

        return result;
    }
}

struct Input {
    s: String,
}

fn main() {
    let inputs = vec![
        Input {
            s: String::from("1001"),
        },
        Input {
            s: String::from("10"),
        },
        Input {
            s: String::from("0000"),
        },
    ];

    for input in inputs {
        let result = Solution::min_changes(input.s);
        println!("{result}");
    }
}

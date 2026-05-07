struct Solution;

#[derive(Debug, PartialEq, Eq)]
pub enum NestedInteger {
    Int(i32),
    List(Vec<NestedInteger>),
}

impl Solution {
    pub fn deserialize(s: String) -> NestedInteger {
        if s.starts_with('[') {
            let mut parts = Vec::<String>::new();
            let mut acc = String::new();
            let mut count = 0;
            let inner = s
                .strip_prefix('[')
                .unwrap()
                .strip_suffix(']')
                .unwrap()
                .chars();
            for ch in inner {
                if count == 0 && ch == ',' {
                    parts.push(acc);
                    acc = String::new();
                    continue;
                }

                if ch == '[' {
                    count += 1;
                } else if ch == ']' {
                    count -= 1;
                }

                acc.push(ch);
            }
            parts.push(acc);

            NestedInteger::List(
                parts
                    .iter()
                    .filter(|s| !s.is_empty())
                    .map(|s| Self::deserialize(s.to_string()))
                    .collect(),
            )
        } else {
            NestedInteger::Int(s.parse().unwrap())
        }
    }
}

struct Input {
    s: String,
}

fn main() {
    let inputs = [
        Input {
            s: "324".to_string(),
        },
        Input {
            s: "[123,[456,[789]]]".to_string(),
        },
    ];

    for input in inputs.into_iter() {
        let result = Solution::deserialize(input.s);
        println!("{:?}", result);
    }
}

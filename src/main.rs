struct Solution {}

impl Solution {
    pub fn modify_string(s: String) -> String {
        let mut result = s.chars().collect::<Vec<char>>();

        for i in 0..result.len() {
            let ch = result[i];
            if ch != '?' {
                continue;
            }

            let mut candidate = std::collections::HashSet::with_capacity(2);
            if i < (result.len() - 1) {
                candidate.insert(result[i + 1]);
            }

            if i > 0 {
                candidate.insert(result[i - 1]);
            }

            result[i] = if !candidate.contains(&'a') {
                'a'
            } else if !candidate.contains(&'b') {
                'b'
            } else {
                'c'
            };
        }

        result.iter().collect()
    }
}

struct Input {
    s: String,
}

fn main() {
    let inputs = [
        Input {
            s: "?zs".to_string(),
        },
        Input {
            s: "ubv?w".to_string(),
        },
    ];

    for input in inputs {
        let result = Solution::modify_string(input.s);
        println!("{:?}", result);
    }
}

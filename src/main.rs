struct Solution;

impl Solution {
    pub fn find_valid_pair(s: String) -> String {
        use std::collections::HashMap;

        let mut counts = HashMap::<char, i32>::new();
        for ch in s.chars() {
            counts.entry(ch).and_modify(|c| *c += 1).or_insert(1);
        }

        for (left, right) in s.chars().zip(s.chars().skip(1)) {
            if left == right {
                continue;
            }

            let lcount = counts.get(&left).copied().unwrap_or_default();
            let rcount = counts.get(&right).copied().unwrap_or_default();
            let lnum = (left as i32) - ('0' as i32);
            let rnum = (right as i32) - ('0' as i32);

            if (lnum == lcount) && (rnum == rcount) {
                return format!("{}{}", left, right);
            }
        }

        String::new()
    }
}

struct Input {
    s: String,
}

fn main() {
    let inputs = [
        Input {
            s: "2523533".to_string(),
        },
        Input {
            s: "221".to_string(),
        },
        Input {
            s: "22".to_string(),
        },
    ];

    for input in inputs {
        let result = Solution::find_valid_pair(input.s);
        println!("{:?}", result);
    }
}

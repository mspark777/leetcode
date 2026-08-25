struct Solution;

impl Solution {
    pub fn reorder_log_files(logs: Vec<String>) -> Vec<String> {
        use std::cmp::Ordering;

        let mut logs = logs;

        logs.sort_by(|a, b| {
            let a_split = a.find(' ').unwrap();
            let b_split = b.find(' ').unwrap();

            let (a_id, a_log) = a.split_at(a_split);
            let (b_id, b_log) = b.split_at(b_split);

            let a_is_digit = Self::is_digit_log(a);
            let b_is_digit = Self::is_digit_log(b);

            match (a_is_digit, b_is_digit) {
                (true, true) => Ordering::Equal,
                (false, false) => a_log.cmp(b_log).then_with(|| a_id.cmp(b_id)),
                (true, false) => Ordering::Greater,
                (false, true) => Ordering::Less,
            }
        });

        logs
    }

    fn is_digit_log(s: &str) -> bool {
        s.split_whitespace()
            .nth(1)
            .unwrap()
            .chars()
            .next()
            .unwrap()
            .is_ascii_digit()
    }
}

struct Input {
    logs: Vec<String>,
}

fn main() {
    let inputs = [Input {
        logs: [
            "dig1 8 1 5 1",
            "let1 art can",
            "dig2 3 6",
            "let2 own kit dig",
            "let3 art zero",
        ]
        .map(|v| v.to_string())
        .to_vec(),
    }];

    for input in inputs {
        let result = Solution::reorder_log_files(input.logs);
        println!("{:?}", result);
    }
}

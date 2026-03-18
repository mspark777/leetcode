use std::cmp::Ordering;

struct Solution;

impl Solution {
    pub fn validate_coupons(
        code: Vec<String>,
        business_line: Vec<String>,
        is_active: Vec<bool>,
    ) -> Vec<String> {
        let mut list = code
            .into_iter()
            .zip(business_line)
            .zip(is_active)
            .filter(|((c, b), a)| Self::valid_code(c) && Self::valid_business(b) && *a)
            .map(|((c, b), _)| (c, b))
            .collect::<Vec<(String, String)>>();
        list.sort_by(Self::sort_by);
        list.into_iter().map(|(code, _)| code).collect()
    }

    fn valid_code(code: &str) -> bool {
        !code.is_empty() && code.chars().find(Self::check_invalid_code).is_none()
    }

    fn check_invalid_code(ch: &char) -> bool {
        let lower = 'a'..='z';
        let upper = 'A'..='Z';
        let digit = '0'..='9';
        let under = '_';

        let ok = lower.contains(ch) || upper.contains(ch) || digit.contains(ch) || (*ch == under);
        !ok
    }

    fn valid_business(line: &str) -> bool {
        ["electronics", "grocery", "pharmacy", "restaurant"].contains(&line)
    }

    fn sort_by(left: &(String, String), right: &(String, String)) -> Ordering {
        match left.1.cmp(&right.1) {
            Ordering::Equal => left.0.cmp(&right.0),
            order => order,
        }
    }
}

struct Input {
    code: Vec<String>,
    business_line: Vec<String>,
    is_active: Vec<bool>,
}

fn main() {
    let inputs = [
        Input {
            code: ["SAVE20", "", "PHARMA5", "SAVE@20"]
                .map(|s| s.to_string())
                .to_vec(),
            business_line: ["restaurant", "grocery", "pharmacy", "restaurant"]
                .map(|s| s.to_string())
                .to_vec(),
            is_active: [true, true, true, true].to_vec(),
        },
        Input {
            code: ["GROCERY15", "ELECTRONICS_50", "DISCOUNT10"]
                .map(|s| s.to_string())
                .to_vec(),
            business_line: ["grocery", "electronics", "invalid"]
                .map(|s| s.to_string())
                .to_vec(),
            is_active: [false, true, true].to_vec(),
        },
    ];

    for input in inputs {
        let result = Solution::validate_coupons(input.code, input.business_line, input.is_active);
        println!("{:?}", result);
    }
}

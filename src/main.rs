struct Solution;

impl Solution {
    pub fn camel_match(queries: Vec<String>, pattern: String) -> Vec<bool> {
        queries
            .into_iter()
            .map(|query| Self::matches(query.as_str(), pattern.as_str()))
            .collect()
    }

    fn matches(query: &str, pattern: &str) -> bool {
        let query = query.chars();
        let mut pattern = pattern.chars().peekable();
        for q in query {
            if Some(&q) == pattern.peek() {
                pattern.next();
            } else if q.is_uppercase() {
                return false;
            }
        }
        pattern.next().is_none()
    }
}

struct Input {
    n: i32,
}

fn main() {
    let inputs = [Input { n: 3 }];

    for input in inputs {
        let result = Solution::base_neg2(input.n);
        println!("{:?}", result);
    }
}

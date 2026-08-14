struct Solution;

impl Solution {
    pub fn ambiguous_coordinates(s: String) -> Vec<String> {
        let s = s.chars().skip(1).take(s.len() - 2).collect::<Vec<char>>();
        let mut result = Vec::<String>::new();
        for i in 1..s.len() {
            for x in Self::candidates(&s[..i]) {
                for y in Self::candidates(&s[i..]) {
                    result.push(format!("({}, {})", x, y));
                }
            }
        }
        result
    }

    fn candidates(v: &[char]) -> Vec<String> {
        let mut candidates = Vec::<String>::new();
        let n = v.len();
        for i in 0..n {
            let s: String = match i {
                0 => v.iter().collect(),
                _ => v[..i]
                    .iter()
                    .chain(std::iter::once(&'.'))
                    .chain(v[i..].iter())
                    .collect(),
            };

            if !((s != "0" && s.starts_with('0') && !s.starts_with("0."))
                || (s.contains('.') && s.ends_with('0')))
            {
                candidates.push(s);
            }
        }

        candidates
    }
}

struct Input {
    s: String,
}

fn main() {
    let inputs = [Input {
        s: "(123)".to_string(),
    }];

    for input in inputs {
        let result = Solution::ambiguous_coordinates(input.s);
        println!("{:?}", result);
    }
}

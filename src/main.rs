struct Solution {}

impl Solution {
    pub fn sort_sentence(s: String) -> String {
        let mut result = s.split_whitespace().collect::<Vec<_>>();
        result.sort_unstable_by_key(|s| s.chars().last().unwrap() as u8);
        result
            .iter()
            .map(|s| s[..s.len() - 1].to_string())
            .collect::<Vec<_>>()
            .join(" ")
    }
}

struct Input {
    s: String,
}

fn main() {
    let inputs = [
        Input {
            s: "is2 sentence4 This1 a3".to_string(),
        },
        Input {
            s: "Myself2 Me1 I4 and3".to_string(),
        },
    ];

    for input in inputs {
        let result = Solution::sort_sentence(input.s);
        println!("{:?}", result);
    }
}

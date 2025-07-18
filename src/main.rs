struct Solution {}

impl Solution {
    pub fn find_ocurrences(text: String, first: String, second: String) -> Vec<String> {
        let mut chunks = text.split(' ');
        let mut f = if let Some(s) = chunks.next() {
            s
        } else {
            return vec![];
        };
        let mut s = if let Some(s) = chunks.next() {
            s
        } else {
            return vec![];
        };

        let mut result = Vec::<String>::new();
        for chunk in chunks {
            if (f == first) && (s == second) {
                result.push(chunk.to_string());
            }

            f = s;
            s = chunk;
        }

        result
    }
}

struct Input {
    text: String,
    first: String,
    second: String,
}

fn main() {
    let inputs = vec![
        Input {
            text: "alice is a good girl she is a good student".to_string(),
            first: "a".to_string(),
            second: "good".to_string(),
        },
        Input {
            text: "we will we will rock you".to_string(),
            first: "we".to_string(),
            second: "will".to_string(),
        },
        Input {
            text: "we we we we will rock you".to_string(),
            first: "we".to_string(),
            second: "we".to_string(),
        },
    ];

    for input in inputs {
        let result = Solution::find_ocurrences(input.text, input.first, input.second);
        println!("{:?}", result);
    }
}

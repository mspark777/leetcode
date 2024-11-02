struct Solution {}

impl Solution {
    pub fn is_circular_sentence(sentence: String) -> bool {
        let bytes = sentence.as_bytes();
        if bytes.first().unwrap() != bytes.last().unwrap() {
            return false;
        }

        for i in 1..(bytes.len() - 1) {
            if bytes[i] == (' ' as u8) {
                if bytes[i - 1] != bytes[i + 1] {
                    return false;
                }
            }
        }

        return true;
    }
}

struct Input {
    s: String,
}

fn main() {
    let inputs = vec![
        Input {
            s: "leetcode exercises sound delightful".to_string(),
        },
        Input {
            s: "eetcode".to_string(),
        },
        Input {
            s: "Leetcode is cool".to_string(),
        },
    ];

    for input in inputs {
        let result = Solution::is_circular_sentence(input.s);
        println!("{result}");
    }
}

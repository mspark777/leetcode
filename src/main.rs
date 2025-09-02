struct Solution {}

impl Solution {
    pub fn truncate_sentence(s: String, k: i32) -> String {
        s.split_whitespace()
            .take(k as usize)
            .collect::<Vec<&str>>()
            .join(" ")
    }
}

struct Input {
    s: String,
    k: i32,
}

fn main() {
    let inputs = [
        Input {
            s: "Hello how are you Contestant".to_string(),
            k: 4,
        },
        Input {
            s: "What is the solution to this problem".to_string(),
            k: 4,
        },
        Input {
            s: "chopper is not a tanuki".to_string(),
            k: 5,
        },
    ];

    for input in inputs {
        let result = Solution::truncate_sentence(input.s, input.k);
        println!("{:?}", result);
    }
}

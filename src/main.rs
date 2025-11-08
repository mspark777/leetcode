struct Solution {}

impl Solution {
    pub fn closest_target(words: Vec<String>, target: String, start_index: i32) -> i32 {
        let n = words.len() as i32;
        let mut shortest = i32::MAX;

        for (i, word) in words.iter().enumerate() {
            if word.as_str() != target.as_str() {
                continue;
            }

            let i = i as i32;
            let to_target = (start_index - i).abs();
            let to_starting = (n - to_target).abs();

            shortest = shortest.min(to_target).min(to_starting);
        }

        match shortest {
            i32::MAX => -1,
            _ => shortest,
        }
    }
}

struct Input {
    words: Vec<String>,
    target: String,
    start_index: i32,
}

fn main() {
    let inputs = [
        Input {
            words: ["hello", "i", "am", "leetcode", "hello"]
                .map(|s| s.to_string())
                .to_vec(),
            target: "hello".to_string(),
            start_index: 1,
        },
        Input {
            words: ["a", "b", "leetcode"].map(|s| s.to_string()).to_vec(),
            target: "leetcode".to_string(),
            start_index: 0,
        },
        Input {
            words: ["i", "eat", "leetcode"].map(|s| s.to_string()).to_vec(),
            target: "ate".to_string(),
            start_index: 0,
        },
    ];

    for input in inputs {
        let result = Solution::closest_target(input.words, input.target, input.start_index);
        println!("{:?}", result);
    }
}

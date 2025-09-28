struct Solution {}

fn to_count(s: &String) -> i32 {
    s.split(' ').filter(|s| !s.is_empty()).count() as i32
}

impl Solution {
    pub fn most_words_found(sentences: Vec<String>) -> i32 {
        sentences.iter().map(to_count).max().unwrap()
    }
}

struct Input {
    sentences: Vec<String>,
}

fn main() {
    let inputs = [
        Input {
            sentences: [
                "alice and bob love  leetcode",
                "i think so too",
                "this is great thanks   very much",
            ]
            .map(|s| s.to_string())
            .to_vec(),
        },
        Input {
            sentences: ["please wait", "continue to fight", "continue to win"]
                .map(|s| s.to_string())
                .to_vec(),
        },
    ];

    for input in inputs {
        let result = Solution::most_words_found(input.sentences);
        println!("{:?}", result);
    }
}

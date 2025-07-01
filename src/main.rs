struct Solution {}

impl Solution {
    pub fn possible_string_count(word: String) -> i32 {
        word.chars()
            .zip(word.chars().skip(1))
            .fold(1, |acc, (left, right)| match left == right {
                true => acc + 1,
                _ => acc,
            })
    }
}

struct Input {
    word: &'static str,
}

fn main() {
    let inputs = vec![
        Input { word: "abbcccc" },
        Input { word: "abcd" },
        Input { word: "aaaa" },
    ];

    for input in inputs {
        let result = Solution::possible_string_count(input.word.to_string());
        println!("{:?}", result);
    }
}

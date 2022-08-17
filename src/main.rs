use std::collections::HashSet;

struct Solution {}
impl Solution {
    pub fn unique_morse_representations(words: Vec<String>) -> i32 {
        let morses = vec![
            ".-", "-...", "-.-.", "-..", ".", "..-.", "--.", "....", "..", ".---", "-.-", ".-..",
            "--", "-.", "---", ".--.", "--.-", ".-.", "...", "-", "..-", "...-", ".--", "-..-",
            "-.--", "--..",
        ];

        let mut seen = HashSet::<String>::new();
        for word in words.iter() {
            let mut codes = Vec::<String>::new();
            for ch in word.bytes() {
                let i = (ch - b'a') as usize;
                codes.push(morses[i].to_string());
            }

            let morse = codes.join("");
            seen.insert(morse);
        }

        return seen.len() as i32;
    }
}

struct Input {
    words: Vec<&'static str>,
}

fn main() {
    let inputs: Vec<Input> = vec![
        Input {
            words: vec!["gin", "zen", "gig", "msg"],
        },
        Input { words: vec!["a"] },
    ];

    for input in inputs {
        let words: Vec<String> = input.words.iter().map(|w| w.to_string()).collect();
        let result = Solution::unique_morse_representations(words);
        println!("{:?}", result);
    }
}

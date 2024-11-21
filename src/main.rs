struct Solution {}

impl Solution {
    pub fn to_goat_latin(sentence: String) -> String {
        let words: Vec<String> = sentence.split(" ").map(|s| s.to_string()).collect();
        let mut chunks = Vec::<String>::new();

        for (i, word) in words.iter().enumerate() {
            let done = Self::process_word(word, i + 1);
            chunks.push(done);
        }

        return chunks.join(" ");
    }

    fn is_first_char_vowel(ch: char) -> bool {
        return match ch {
            'a' | 'e' | 'i' | 'o' | 'u' | 'A' | 'E' | 'I' | 'O' | 'U' => true,
            _ => false,
        };
    }

    fn process_word(word: &String, idx: usize) -> String {
        let first = word.chars().nth(0).unwrap();
        let append: String = (vec!['a'; idx]).iter().collect();
        return if Self::is_first_char_vowel(first) {
            format!("{}ma{}", word, append)
        } else {
            let sub: String = word.chars().skip(1).collect();
            format!("{}{}ma{}", sub, first, append)
        };
    }
}

struct Input {
    sentence: &'static str,
}

fn main() {
    let inputs = vec![
        Input {
            sentence: "I speak Goat Latin",
        },
        Input {
            sentence: "The quick brown fox jumped over the lazy dog",
        },
    ];

    for input in inputs {
        let result = Solution::to_goat_latin(input.sentence.to_string());
        println!("{result}");
    }
}

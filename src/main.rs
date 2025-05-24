struct Solution {}

impl Solution {
    pub fn find_words_containing(words: Vec<String>, x: char) -> Vec<i32> {
        let mut result = Vec::<i32>::with_capacity(words.len());
        for (i, word) in words.iter().enumerate() {
            if word.contains(x) {
                result.push(i as i32);
            }
        }

        return result;
    }
}

struct Input {
    words: Vec<&'static str>,
    x: char,
}

fn main() {
    let inputs = vec![
        Input {
            words: ["leet", "code"].to_vec(),
            x: 'e',
        },
        Input {
            words: ["abc", "bcd", "aaaa", "cbc"].to_vec(),
            x: 'a',
        },
        Input {
            words: ["abc", "bcd", "aaaa", "cbc"].to_vec(),
            x: 'z',
        },
    ];

    for input in inputs {
        let result = Solution::find_words_containing(
            input.words.iter().map(|s| s.to_string()).collect(),
            input.x,
        );
        println!("{:?}", result);
    }
}

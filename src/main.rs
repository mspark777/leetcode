struct Solution;

impl Solution {
    pub fn map_word_weights(words: Vec<String>, weights: Vec<i32>) -> String {
        let mut mapped_list = Vec::<char>::with_capacity(words.len());

        for word in words {
            let weight = word
                .chars()
                .fold(0, |acc, ch| acc + Self::weight(&weights, ch))
                % 26;

            let ch = Self::mapped(weight);
            mapped_list.push(ch);
        }

        mapped_list.into_iter().collect()
    }

    fn weight(weights: &[i32], ch: char) -> i32 {
        let idx = ((ch as u8) - b'a') as usize;
        weights[idx]
    }

    fn mapped(weight: i32) -> char {
        let idx = (25 - weight) as u8;
        (b'a' + idx) as char
    }
}

struct Input {
    words: Vec<String>,
    weights: Vec<i32>,
}

fn main() {
    let inputs = [
        Input {
            words: ["abcd", "def", "xyz"].map(|s| s.to_string()).to_vec(),
            weights: [
                5, 3, 12, 14, 1, 2, 3, 2, 10, 6, 6, 9, 7, 8, 7, 10, 8, 9, 6, 9, 9, 8, 3, 7, 7, 2,
            ]
            .to_vec(),
        },
        Input {
            words: ["abcd", "def", "xyz"].map(|s| s.to_string()).to_vec(),
            weights: [
                5, 3, 12, 14, 1, 2, 3, 2, 10, 6, 6, 9, 7, 8, 7, 10, 8, 9, 6, 9, 9, 8, 3, 7, 7, 2,
            ]
            .to_vec(),
        },
        Input {
            words: ["abcd", "def", "xyz"].map(|s| s.to_string()).to_vec(),
            weights: [
                5, 3, 12, 14, 1, 2, 3, 2, 10, 6, 6, 9, 7, 8, 7, 10, 8, 9, 6, 9, 9, 8, 3, 7, 7, 2,
            ]
            .to_vec(),
        },
    ];

    for input in inputs {
        let result = Solution::map_word_weights(input.words, input.weights);
        println!("{:?}", result);
    }
}

struct Solution {}

impl Solution {
    pub fn num_ways(words: Vec<String>, target: String) -> i32 {
        let target = target.chars().collect::<Vec<char>>();
        let word_length = words[0].len();
        let target_length = target.len();
        let m = 1_000_000_007;
        let mut char_frequencies = vec![vec![0; 26]; word_length];

        for word in words.iter() {
            for (j, ch) in word.chars().enumerate() {
                let i = ((ch as u8) - ('a' as u8)) as usize;
                char_frequencies[j][i] += 1;
            }
        }

        let mut prev_counts = vec![0i64; target_length + 1];
        let mut curr_counts = vec![0i64; target_length + 1];
        prev_counts[0] = 1;

        for curr_word in 1..=word_length {
            curr_counts = prev_counts.clone();
            for curr_target in 1..=target_length {
                let cur_pos = ((target[curr_target - 1] as u8) - ('a' as u8)) as usize;
                curr_counts[curr_target] +=
                    (char_frequencies[curr_word - 1][cur_pos] * prev_counts[curr_target - 1]) % m;
                curr_counts[curr_target] %= m;
            }

            prev_counts = curr_counts.clone();
        }

        return curr_counts[target_length] as i32;
    }
}

struct Input {
    words: Vec<&'static str>,
    target: &'static str,
}

fn main() {
    let inputs = vec![
        Input {
            words: vec!["acca", "bbbb", "caca"],
            target: "aba",
        },
        Input {
            words: vec!["abba", "baab"],
            target: "bab",
        },
    ];

    for input in inputs {
        let result = Solution::num_ways(
            input.words.iter().map(|s| s.to_string()).collect(),
            input.target.to_string(),
        );
        println!("{result:?}");
    }
}

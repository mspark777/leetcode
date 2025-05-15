struct Solution {}

impl Solution {
    pub fn get_longest_subsequence(words: Vec<String>, groups: Vec<i32>) -> Vec<String> {
        let mut result = Vec::<String>::new();
        for (i, word) in words.iter().enumerate() {
            if i == 0 {
                result.push(word.clone());
            } else if groups[i] != groups[i - 1] {
                result.push(word.clone());
            }
        }

        return result;
    }
}

struct Input {
    words: Vec<&'static str>,
    groups: Vec<i32>,
}

fn main() {
    let inputs = vec![
        Input {
            words: vec!["c"],
            groups: vec![0],
        },
        Input {
            words: vec!["e", "a", "b"],
            groups: vec![0, 0, 1],
        },
        Input {
            words: vec!["a", "b", "c", "d"],
            groups: vec![1, 0, 1, 1],
        },
    ];

    for input in inputs {
        let result = Solution::get_longest_subsequence(
            input.words.iter().map(|s| s.to_string()).collect(),
            input.groups,
        );
        println!("{result:?}");
    }
}

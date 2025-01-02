struct Solution {}

impl Solution {
    pub fn vowel_strings(words: Vec<String>, queries: Vec<Vec<i32>>) -> Vec<i32> {
        let mut result = vec![0; queries.len()];
        let vowels = std::collections::HashSet::<char>::from(['a', 'e', 'i', 'o', 'u']);
        let mut prefix_sum = vec![0; words.len()];
        let mut sum = 0;

        for (i, word) in words.iter().enumerate() {
            let chars = word.chars().collect::<Vec<char>>();
            let first = chars.first().unwrap();
            let last = chars.last().unwrap();

            if vowels.contains(&first) && vowels.contains(&last) {
                sum += 1;
            }

            prefix_sum[i] = sum;
        }

        for (i, query) in queries.iter().enumerate() {
            let l = query[0] as usize;
            let r = query[1] as usize;
            if l == 0 {
                result[i] = prefix_sum[r];
            } else {
                result[i] = prefix_sum[r] - prefix_sum[l - 1];
            }
        }

        return result;
    }
}

struct Input {
    words: Vec<&'static str>,
    queries: Vec<Vec<i32>>,
}

fn main() {
    let inputs = vec![
        Input {
            words: vec!["aba", "bcb", "ece", "aa", "e"],
            queries: vec![vec![0, 2], vec![1, 4], vec![1, 1]],
        },
        Input {
            words: vec!["a", "e", "i"],
            queries: vec![vec![0, 2], vec![0, 1], vec![2, 2]],
        },
    ];

    for input in inputs {
        let result = Solution::vowel_strings(
            input.words.iter().map(|s| s.to_string()).collect(),
            input.queries,
        );
        println!("{result:?}");
    }
}

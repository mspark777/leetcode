mod solution;

use solution::Solution;

struct Input {
    words1: Vec<&'static str>,
    words2: Vec<&'static str>,
}

fn strvec(strs: Vec<&'static str>) -> Vec<String> {
    strs.iter().map(|s| String::from(*s)).collect()
}

fn main() {
    let inputs: Vec<Input> = vec![
        Input {
            words1: vec!["amazon", "apple", "facebook", "google", "leetcode"],
            words2: vec!["e", "o"],
        },
        Input {
            words1: vec!["amazon", "apple", "facebook", "google", "leetcode"],
            words2: vec!["l", "e"],
        },
    ];

    for input in inputs {
        let words1 = strvec(input.words1);
        let words2 = strvec(input.words2);
        let result = Solution::word_subsets(words1, words2);
        println!("{:?}", result);
    }
}

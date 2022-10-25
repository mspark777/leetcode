struct Solution {}
impl Solution {
    pub fn array_strings_are_equal(word1: Vec<String>, word2: Vec<String>) -> bool {
        return word1.join("") == word2.join("");
    }
}

struct Input {
    word1: Vec<&'static str>,
    word2: Vec<&'static str>,
}

fn main() {
    let inputs = [
        Input {
            word1: vec!["ab", "c"],
            word2: vec!["a", "bc"],
        },
        Input {
            word1: vec!["a", "cb"],
            word2: vec!["ab", "c"],
        },
        Input {
            word1: vec!["abc", "d", "defg"],
            word2: vec!["abcddefg"],
        },
        Input {
            word1: vec!["abc", "d", "defg"],
            word2: vec!["abcddef"],
        },
    ];

    for Input { word1, word2 } in inputs {
        let word1 = word1.iter().map(|s| s.to_string()).collect();
        let word2 = word2.iter().map(|s| s.to_string()).collect();
        let result = Solution::array_strings_are_equal(word1, word2);
        println!("{result:?}");
    }
}

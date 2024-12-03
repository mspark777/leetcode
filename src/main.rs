struct Solution {}

impl Solution {
    pub fn is_prefix_of_word(sentence: String, search_word: String) -> i32 {
        for (i, word) in sentence.split(" ").enumerate() {
            let mut chars = word.chars();
            let mut found = true;
            for search in search_word.chars() {
                if let Some(ch) = chars.next() {
                    if ch != search {
                        found = false;
                        break;
                    }
                } else {
                    found = false;
                    break;
                }
            }

            if found {
                return (i + 1) as i32;
            }
        }

        return -1;
    }
}

struct Input {
    sentence: &'static str,
    search_word: &'static str,
}

fn main() {
    let inputs = vec![
        Input {
            sentence: "i love eating burger",
            search_word: "burg",
        },
        Input {
            sentence: "this problem is an easy problem",
            search_word: "pro",
        },
        Input {
            sentence: "i am tired",
            search_word: "you",
        },
    ];

    for input in inputs {
        let result =
            Solution::is_prefix_of_word(input.sentence.to_string(), input.search_word.to_string());
        println!("{result}");
    }
}

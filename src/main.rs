use std::collections::HashMap;

struct Solution {}
impl Solution {
    pub fn longest_palindrome(words: Vec<String>) -> i32 {
        let mut counts = HashMap::<String, i32>::new();
        for word in words.iter() {
            if let Some(count) = counts.get_mut(word) {
                *count += 1;
            } else {
                counts.insert(word.clone(), 1);
            }
        }

        let mut result = 0;
        let mut central = false;

        for (word, count) in counts.iter() {
            let bytes = word.as_bytes();
            let first = bytes[0];
            let second = bytes[1];
            if first == second {
                if (count % 2) == 0 {
                    result += count;
                } else {
                    result += count - 1;
                    central = true;
                }
            } else if first < second {
                let rword = String::from_utf8(vec![second, first]).unwrap();
                if let Some(rcount) = counts.get(&rword) {
                    result += 2 * count.min(rcount);
                }
            }
        }

        if central {
            result += 1;
        }

        return result * 2;
    }
}

fn main() {
    let inputs = [
        vec!["lc", "cl", "gg"],
        vec!["ab", "ty", "yt", "lc", "cl", "ab"],
        vec!["cc", "ll", "xx"],
    ];

    for words in inputs {
        let words = words.iter().map(|s| s.to_string()).collect();
        let result = Solution::longest_palindrome(words);
        println!("{result}");
    }
}
